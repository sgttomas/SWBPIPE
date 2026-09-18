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

Return of UX-SPEC-01, retained under `../instances/UX-SPEC/` and accepted by ROOT on 2026-09-18 against the brief's acceptance section: the eleven sections are present in the brief's order; every surface of the brief §4 and every frame state has its behaviour with state tables where states differ; every table has its columns, keyboard model and validation; the proposal lifecycle, the Checked mark and the agent's checks are specified end to end (§6); the seventeen disclosures each carry surface, trigger and wording source with file and line (§9.1); the operations map has 277 rows over eighteen surfaces with 29 classed gaps; the open questions are behaviours (§11); RETURN is complete. ROOT's checks before accepting: no absolute path in any file; a word-level scan finds no forbidden word other than the registered name "Review/signoff block" and one identifier; every one of the map's 329 file-and-line citations names a file that exists and a line inside it, and of 275 identifier-to-line pairs 266 carry the identifier within three lines of the cited line, the other nine citing a location inside the named component. ROOT's decisions on the interpretations the child offered for decision (RETURN §4 items 7, 8, 11, 12, 13 and §5 item 5): the chip after a failed run follows the run record, and the frame that draws "Model incomplete" after a non-convergence is corrected in the next mocks pass; the M-04 trigger follows the constraints sheet's wording; the interpretations listed under §4 item 11 stand as specified; the Type column offers the engine's element kinds only and the design system's table of the file's kinds is aligned in its next revision; the acceptance sentence's three homes, one per surface class, are the reading ROOT confirms and carries to the decision packet as items 2 and 8; product copy follows the product's existing spelling of "Analyze", the copy rule to be stated in the design system's next revision. The specification cites design system V1 by name and section; where the V1.1 revision renames or renumbers, the specification is reconciled after V1.1 is accepted. All files are byte for byte the child's.

| Return file | SHA-256 |
|---|---|
| `UX_SPEC_V1.md` | `1406f4012a8b8c84977d44d9c3e759dfa1110e4c5e6ed5a565ac7de57f77a09e` |
| `OPERATIONS_MAP.md` | `891861bba08a52c43d05d7704b4d54dacd91a271a2f2d95dff39db53ea2b5105` |
| `RETURN.md` | `d47553e4c5444102021825bfb9cf74374965e984c998b35c767102e527147178` |

Return of RESEARCH-F, retained at `../instances/RESEARCH/F_packet_bindings.md` and accepted by ROOT on 2026-09-18 against the brief's acceptance section: each of the nine items carries its governed text verbatim with source, its enforcement points with what each checks, its occurrences classified as registered placement, ordinary string or historical record, its redesign placements by file, and the owner act a change requires; the summary table is complete (96 enforcement points); unknowns are written as TBD. ROOT's check before accepting found one false statement in the first return: that the phrase "Human review required" has no occurrence under the desktop source. ROOT's own search found it inside two diagnostic remediation strings and sent the child the finding. The child re-ran the search for all six short-label phrases as case-insensitive substrings, corrected §0.3 row D-7, §4.2, §4.3, §4.5, §4.6, the summary row and the return, recorded the cause (a doubled path prefix with the error suppressed) in a "Correction after ROOT's check" note, and in doing so found a material fact the first pass had missed: the workspace status pills already render two curated short labels, "Review required" and "Inputs needed", with a lowercased de-underscored form for the other four tokens and the raw token beneath, pinned by seven Vitest assertions; ROOT verified the mapping at `apps/desktop/src/App.tsx:3766-3773`. One declared deviation from the brief's citation rule stands: `DESIGN_SYSTEM_V1.md` is cited by section and quotation, not by line, because the DESIGN-SYSTEM-02 revision was rewriting that file while the inventory ran; the return states this in its §0 and nothing the nine items depend on lives in that file. The retained file is byte for byte the child's corrected return.

| Return file | SHA-256 |
|---|---|
| `F_packet_bindings.md` | `2c9fcb1716981123da8f9c78f8c70d6db77feb931b70936073159530bfb0292b` |

Return of DESIGN-SYSTEM-02, retained in place under `../instances/DESIGN-SYSTEM/` (V1.1 replaces V1; the V1 hashes above remain the record of what V1 was) and accepted by ROOT on 2026-09-18 against the brief's acceptance section: all fourteen decisions are applied and traceable in the new §9 (49 rows, each with its source); every MOCKS_V1 §1 to §3 item is adopted or overruled with a reason (P-5 and P-9 superseded by decisions 1 and 12, P-8 adopted for the frames and overruled as a preset, the s5 cell popover overruled by decision 14); the six token gaps are closed by ten new tokens with light and dark values and contrast findings; the dark result scale is re-anchored and re-validated with the dataviz ordinal validator in both themes; the child's agreement check reports zero differences between document, token file and specimen; the specimen renders offline in both themes; RETURN is complete. ROOT re-ran the agreement check (98 colour tokens, 146 contrast rows, `problems: []`) and the child's render script headlessly at 1440 and 720 in light and dark with the manual switch (0 requests, 0 console issues, no horizontal scroll) before accepting. ROOT's review sent the child two points, applied as §9 rows 48 and 49: the Both-view narrow case follows the UX specification's rule (the agent column collapses to its strip first, then a slide-over for that window size only; the canvas is never collapsed to an edge tab), and product copy uses Canadian English spelling. ROOT's decisions on the child's uncertainties: the reading of decision 11 (the ramp's bottom one step down and its top under the edge line) is confirmed as the decision's purpose; "Review/signoff block" is registered text (`docs/PRD.md:1242`); the design system's 220 px canvas minimum governs the geometry and the UX specification's §10.9 floors are reconciled to it in that document's next revision; the edge line's 1.10:1 on the brightest dark step is carried as an input to the contrast open item, and the next revision may consider an edge token that switches by fill lightness. The `tools/` scripts locate the validator and Playwright by argument or relative path; no absolute path exists in the instance.

| Return file | SHA-256 |
|---|---|
| `DESIGN_SYSTEM_V1.md` | `34ba3f1fadacbd1c3854e7bb654098b5525e25b9734b4106dfdd39fc94f0cf1f` |
| `tokens.json` | `b6a382727728845c7cafe3d9e66b4bf87df888c9a6bebaae8c1f9f59dbbafe59` |
| `specimen.html` | `5be4f482069225de0f35c2bbce57e4046d4ebae6095a9ed8970f7bb6611ef6b7` |
| `RETURN.md` | `82fa203f5b83a83cb06cf3e7eb9bdf91f53f03d995c9b0feab1b94319587bc56` |
| `tools/agree.mjs` | `f2794b78d648a09ec2df49cd63e526f1055e0c9ff8782adae42430338c45fac3` |
| `tools/contrast.mjs` | `e0afa387535a5f4070d812977db010f1652205d1a7805f4151f5633dbd7c264b` |
| `tools/gen.mjs` | `8179b401e3f230634ef71f50995e9c96e52b4580d8adbb502a595433a7b6a998` |
| `tools/palette.mjs` | `57d9e34049cd391fe3d06426f023968ac5b3b950aa2d86451a1205d123f40131` |
| `tools/render.mjs` | `edc4aa2f85a635d1a2a889f4b2697419947cff4a546908543abbf530ce28267c` |
| `tools/splice.mjs` | `82872b084ffbda52d58defbdfc4f1e34ce16be33571594819a2ccf8ecde9563b` |

## Phase 3 second mocks pass, sealed before launch

Sealed by ROOT on 2026-09-18 after accepting design system V1.1 and the UX specification. One child, working alone; parentage ROOT; write target instruction-asserted (`../instances/MOCKS/`).

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `MOCKS-02_regeneration.md` | `9a0c67e0b129b7e85e1e12b0fdb298afd716fcde5dd26827e4807fbf9aa59c2f` | 2026-09-18T11:15Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/MOCKS/` (frames regenerated in place; `MOCKS_V2.md`) |

Return of MOCKS-02, retained in place under `../instances/MOCKS/` (the frames, build sources, tools and shots replace the first pass's; `MOCKS_V1.md`, `sample_model.md` and the first pass's hashes above remain its record) and accepted by ROOT on 2026-09-18 against the brief's acceptance section: the fifteen frames are regenerated from `tokens.json` 1.1 and V1.1's components; every second-pass state of V1.1 §8 is drawn, including the new `s4_both_light_column`; the two decision-aid frames exist and carry their label in the caption bar and the toolbar band; the failed-run frame shows no chip, as the specification's §5.4 item 5 says pending its §11 Q11; the child's lint finds each disclosure once where V1.1 §7.4 places it (the acceptance sentence in the four results and Review frames only; its short variant in the item 2 aid frame only; the maturity sentence in state 1 and twice in the item 1 aid frame) and no forbidden word at any viewport; every value traces to the token file or is a listed gap; `MOCKS_V2.md` records every change with its source, eleven departures with reasons, fourteen frame decisions, eight questions (Q-15 to Q-22) and six gaps (G-7 to G-12); the eighteen 1:1 shots, `index.png` and `report.json` exist; RETURN is complete. ROOT's checks before accepting: no absolute path and no external reference in any shipped file; nothing changed outside `instances/MOCKS/`; six frames judged by eye from their shots (`s4_both_light_column`, both decision aids, `s7_both_dark`, `s6_both_light`, `s9_table_light`); one frame opened in the App's browser pane, which renders a local file as a static snapshot, and confirmed styled with no network request. ROOT's reading of the child's uncertainties: the item 2 aid frame correctly draws option A as the caption in place of the disclosure's sentence, since option A gives the results surface class one placement; the item 1 aid frame's placement left of the units control is one of the two readings and the owner sees it. The eight questions are carried to the revision pass that follows the owner's ruling on D-71; none blocks the packet. All files are byte for byte the child's.

| Return file | SHA-256 |
|---|---|
| `MOCKS_V2.md` | `f1883ffd04acaeb8974ff0270745eef7dd7887b6d1f020f8c4410aae832f42eb` |
| `RETURN.md` | `a440ffd114aee6c5ebfde1bd70868fa6e522a97201942567e37bef90ca25d4ab` |
| `frames/mocks.css` | `81cb4f5f218949136d649873e4d95309e582ce1aadde52f493b2a8a26d2a4803` |
| `frames/tokens.css` | `f49d23c696ff343dd683c14533cb21f8b93b38ed1738f121fb45be78cd1d4dca` |
| `shots/report.json` | `25e3c0398d91b82674facf9fae0998f4b1760085ee9650c35c9f81a6ea5c4a09` |

The eighteen 1:1 stage screenshots and `index.png` under `shots/` are retained as the child produced them; `report.json` does not record their hashes, so ROOT recorded them here at retention (they are also derivable by re-running `tools/render.mjs`).

| Retained screenshot | SHA-256 |
|---|---|
| `shots/d71_item1_status_bar_light.png` | `672ac46e0adf3c292b856c1910fcb660fd979492d5440259ad011b4cab98a9e9` |
| `shots/d71_item2_results_caption_light.png` | `f3b66d0b1e7a761b3f02d56ac4427e40204f65e44e80423febb26c2b4b613519` |
| `shots/index.png` | `3bf2c1d6a9ff7dafe7e23e6f1997c0dc07cda92bd9cade3c836d84565fbc1985` |
| `shots/s1_table_light.png` | `da986a87be947eebd603bc3a6b8c76bf126a3f7b4a7344efc62dd96ab1b9ff86` |
| `shots/s2_model_dark.png` | `0bf84544c86e626d48654fab898ffc231b6d7a00fb915403445ed0b8d42a0c90` |
| `shots/s2_model_light.png` | `b805a40913c281c643998576780484d14a25fb2704e8feb3862631f44acf7544` |
| `shots/s3_table_light.png` | `a89f8b0b4a7af92280c209462d88b69fce1856afdf392d19680e1f8f5824bc4b` |
| `shots/s4_both_light.png` | `cc9b64bdb6b2b441d9c019e689be6fff8ec516726c5a8533a9ef04f51b4bd801` |
| `shots/s4_both_light_column.png` | `2bbe912393b276f3fd195c744836bf4c47dde645a991b674ebae1d5903ca2b48` |
| `shots/s4_table_light.png` | `45525f76e5b8f0609b3a852dd6ad917637c6e1593fe0a58bed609b28079d11dc` |
| `shots/s5_table_light.png` | `50247a5240b6e64df5c33cbb44fcfacd39147a35b4c045c674f7348d6762006e` |
| `shots/s6_both_light.png` | `bb9ae8046bc6c66cee9c726be969f71119bca22cc8a06f8b41b68cc6e85686f3` |
| `shots/s7_both_dark.png` | `6e1a7e18ea3127b1e814a4c36eddb976d26cf78784aa34e5ebfe5f5f257a048c` |
| `shots/s7_both_light.png` | `24f77d83a89837750b6498adb8ae8296bf35520a7f6aa006e4f68a682e21df1d` |
| `shots/s7_table_light.png` | `67457750ea18bdff2d653b49fe6508057f5c5218b5f00bcb253ca5fe93a75dba` |
| `shots/s8_model_light.png` | `2ee3f5b3aab85c196968a7a2768417f9dbf1129607d0fd043979c6aea69fa6b7` |
| `shots/s8_table_light.png` | `852b00080aab02073495a3be4326ed24d02555e39d5d918b0509c8cf989207d9` |
| `shots/s9_table_dark.png` | `67cf36a9ea18de376a616cac370a6b2907f2dd5c3eea8b7ecfa2cf452d60355e` |
| `shots/s9_table_light.png` | `63449568cfdadf9ad03af32e1164102ab5208fc57ad0d1b043f5053c25e20dc7` |

## Successor ROOT, 2026-09-18: workload classification brief, sealed before launch

From this section on, ROOT is a successor session (Claude Code, `claude-fable-5-1`, HELP_HUMAN), activated by the owner's handoff message recorded at `../instances/ROOT/SUCCESSOR_ACTIVATION_2026-09-18.md`, working on branch `codex/swb-ui-design-handoff-reconciliation` from `origin/main` at `451c5f595e0488a0d6064d9b0f972c5e8fc1d09d`. The program continues in its original design harness, so the established allocation holds: Opus for objective or quantitative work, Fable for design-quality and judgment work. Mechanism is unchanged: Claude Code `Agent` tool, `general-purpose` subagent type, harness-native descendants of ROOT, not Chirality-managed `delegate_agent` sessions; write targets instruction-asserted, not sandbox-enforced; supplied context is the sealed brief only; Type 2 children do not delegate.

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `RESEARCH-G_rendering_workload_classification.md` | `0c9ea3a46d5115588fdab3c891d47c8e76b204e87cc63d7d98e511c399ba23e7` | 2026-09-18T12:52Z | `opus` (Claude Opus 5) | TASK, bounded read-only research, working alone; tools: file reading and search only by instruction, no build, test or benchmark | `../instances/RESEARCH/G_rendering_workload_classification.md` |

Return of RESEARCH-G, retained at `../instances/RESEARCH/G_rendering_workload_classification.md` byte for byte and accepted by ROOT on 2026-09-18 against the brief's acceptance section: all 50 rows (34 for the rendering brief's §2 with items split where parts differ, 8 for §3, 8 for §4) carry exactly one class (10 observable now, 8 needing a bounded harness adaptation, 32 needing implementation first); class 1 and 2 rows cite rendering and instrument code; of the 32 class 3 rows, 12 show a search and 20 argue absence from cited code or by reference to another row, which ROOT accepts as meeting the brief's purpose though not its letter; unavailable measurements are listed (§3.2); no absolute path; the working tree showed no file touched other than the return. ROOT's checks: six citations read against the source and found as described (the boundary validator's device pixel ratio 2 and 1440 × 920 requirement at `apps/desktop/e2e/ui-foundation/characterization-commands.ts:50-53`; memory recorded as unavailable at `full-cohort-controller.ts:905-906`; the label overlap test at `PipeViewport.tsx:1347-1355`; per-instance recolour at `viewportResource.ts:981-985`; the pixel-ratio cap at `:407`; isolate through the hidden-key mask), and two absence claims re-run by ROOT with the same zero result (no caller of `pipeMesh`, `componentMesh` or `deformedPipeMesh` under `apps/desktop/src`; no consumer of the `deformation-overlay` fixtures other than the generator and the manifest). The child's seven uncertainties stand as stated, the first (whether the in-product preview solves the benchmark fixtures) carried into the observation brief as a question only a runtime reading answers. The host reported the run as complete; the host's own attribution of the model that ran is not independently verified here.

| Return file | SHA-256 |
|---|---|
| `G_rendering_workload_classification.md` | `fcf6eb88bb43dbc28d32156b3766cf5a388dad0b96316ca8623c470dc0d2cc45` |

## Successor ROOT, 2026-09-18: independent review brief, sealed before launch

A fresh read-only reviewer over the complete branch diff, with no write target and none of the author's context; same mechanism and parentage as RESEARCH-G; supplied context is the sealed brief only. Its return is its final message, which ROOT retains at `../instances/REVIEW/REVIEW-01_RETURN.md`.

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `REVIEW-01_successor_records_review.md` | `7125b03e170d31d3efed4e5993ca25f47d1b2cdb295f38ffdc4fbfa8653ac3e8` | 2026-09-18T13:08Z | `opus` (Claude Opus 5) | TASK, read-only reviewer, working alone | `../instances/REVIEW/REVIEW-01_RETURN.md` (retained by ROOT from the child's final message) |

Correction after REVIEW-01, 2026-09-18. As first written, the RESEARCH-G acceptance paragraph above, the observation brief's status line and the work graph's note said 38 rows in classes of 10, 7 and 21. Those numbers came from the child's final message, which ROOT recorded without counting the retained file. The reviewer counted the file; ROOT recounted and confirms 50 rows in classes of 10, 8 and 32, and corrected the three places. The classification's substance, the observation brief's seven observations and the statement that six of the rendering brief's eight §4 observations name subjects the product does not draw were unaffected. The first wording is in commit `cd3dba125da109f1cc50314874725c154296ef0a`.

Return of REVIEW-01, retained at `../instances/REVIEW/REVIEW-01_RETURN.md` as the child's final message, reviewed SHA `cd3dba125da109f1cc50314874725c154296ef0a`, verdict FINDINGS: one major (the counts above, corrected), two minor (the class 3 searches wording, corrected above; the return's own retention path not yet existing, resolved by retaining it), one trivial (a product identifier's spelling echoed in RESEARCH-G's prose, left as the child wrote it). Checks 1, 3, 4 and 5 (no invented ruling, attribution kept apart, history preserved, no circular prerequisite) passed, with every hash cited in the reconciliation record, D-72 and the observation brief, the three new hashes of this index, the merge SHAs, the PR numbers and the arithmetic recomputed; hashes in this index's earlier sections were outside the review's scope. The corrections went back to the same reviewer, whose backcheck of `202651a09ce87c8d87332f83f2d6e78595ca07db` passed all four items with one trivial finding against its own first return (five hashes in the reconciliation table, not six), recorded in the retained return.

## Successor ROOT, 2026-09-18: the revision pass, sealed brief by brief

Sealed after the owner ruled D-71 in part and answered the frames' questions in session (owner message of 2026-09-18T13:32:54.672Z, SHA-256 `711ca05ef7cb9bcaf6b7f0e6812ad002be58d8f9793aa68a08d0f19bceaf7944`, recorded in the D-71 ruling record). Item 7 of D-71, the Checked mark's classification, is still with the owner and is carried unchanged through the pass. Mechanism, parentage and write-scope assertion as for RESEARCH-G; the briefs run in sequence because each revises from the one before.

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `DESIGN-SYSTEM-03_revision.md` | `e0af1bda062a3ad6c67693ef394c6825cecbea4a3268ebcd4bd6bdf2e2b61be2` | 2026-09-18T13:37Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/DESIGN-SYSTEM/` (V1.2 in place) |

The returns are research inputs to the design program. They are not accepted findings, not decomposition truth and not product claims. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
