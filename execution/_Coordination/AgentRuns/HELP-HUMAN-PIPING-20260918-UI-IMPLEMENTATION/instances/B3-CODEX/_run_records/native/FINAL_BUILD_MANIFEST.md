# Final native build manifest

Candidate commit: `26a7478b5b7e85760a8f9f8847690c548a01f9e5`  
Integration base: `c459a0fa15e8d33613b6b95f529b5d369c707874`  
Bundle identifier: `com.chirality.swbpipe`  
Bundle version: `0.1.0`

The final Tauri build ran while these source bytes were dirty over then-HEAD
`8f9c2d07104d467d635e78b6a405afb17ddc94f4`. They were committed without a
product-byte change as `26a7478b5b7e85760a8f9f8847690c548a01f9e5`.
`git diff --name-only HEAD --` over `apps/desktop/src`, `src-tauri` and `e2e`
returned empty after that commit. For every present file in the table below,
SHA-256 was computed once from the worktree and once from `git show HEAD:path`;
the two values matched.

## Bundle

| Artifact | SHA-256 |
|---|---|
| `src-tauri/target/debug/bundle/macos/SWBPIPE.app/Contents/MacOS/openpipestress-desktop` | `a7d7488abe0b3e7f3ffaacf9fc8156e48a61eb180694075083e9a600509ae484` |
| `src-tauri/target/debug/bundle/macos/SWBPIPE.app/Contents/Info.plist` | `573a9848c4b521dc352ba10ef10298c645702b53f0557e62bade6a1855261fd8` |
| `src-tauri/target/debug/bundle/macos/SWBPIPE.app/Contents/Resources/icon.icns` | `2ddf5cdbfe7c2023349d49fd45efee4debad28e72a4cf8401834aa402f3d238e` |

Tauri embeds `frontendDist` into the executable rather than copying it as
ordinary files under `Contents/Resources`. The exact pre-embed dist tree is
listed next.

## Frontend dist tree embedded by the build

| File under `apps/desktop/dist` | SHA-256 |
|---|---|
| `index.html` | `536bcdb559103f445406f80e95a21090e05d07adf2a73867ad11cdc81092d6b7` |
| `assets/index-CW4hA7Jy.css` | `12c37704142d30668ba3ca040d5708db089f2eb6f9b145965446b9e52efa095e` |
| `assets/index-CfdMWcix.js` | `c04de1921f13ce46fa771ece1b73d1eefa92894bc11a0a8901c025c10f33ddf1` |
| `assets/invented_agent_proposal-Bm57LVt-.js` | `181148caaab83efcd8c916cfd819b2bde82a0e0581ec096b5119ef37164c542c` |
| `assets/invented_demo_rule_pack-dhX9tvT1.js` | `2171fad960204d286467275400ac5d710059b09936dbebde262fe03dc4047a0a` |
| `assets/invented_design_knowledge-H12r-oKQ.js` | `ddb6693d09d218a01bc3f31f80680e4bb11e988a851c6e9fe55ba1b10be16040` |
| `assets/invented_mechanics_result-CleTy2JL.js` | `493050a50b346dd1c53de6481da58d3c33d3302c0308f17b49e60584a233be4d` |
| `assets/invented_preview_model-pJA3_nB8.js` | `f21511f5742daf6276173a5b9bcd440d6c3283cacfe83f50a2f857b1a882605a` |
| `assets/vendor-7OC5HNn7.js` | `3d45cc5c629e739f535e3d840a5a5a8302cdde5c8b93ad5aa5a6f28cb16fda76` |
| `assets/vendor-icons-BjC2kMFO.js` | `c16587f1d30af87974f04473da4620f1eb7bfc4f592da3d8a78c8320b0c50733` |
| `assets/vendor-react-u0ysdD_x.js` | `63fc4459119f62f88e22f9114753605a94dc4fb8a870ffca75648df4c68f8901` |
| `assets/vendor-tauri-DlQNAQKj.js` | `ce6c2d2158f4a2b727d1886ef0b5f05bcb63e4d86bde10e05598707dde9b6e73` |
| `assets/vendor-three-hE6SPF4C.js` | `b46ab73b7015fd1a115511a32fccdc8ea9dab4f93f5f0a8f952aecd0e628a834` |
| `self-weight-engine/open_pipe_stress_self_weight_wasm.d.ts` | `634546fac35bc42da31bf903e52a5799429422f44291d901495675bb4da4cc01` |
| `self-weight-engine/open_pipe_stress_self_weight_wasm.js` | `ea2b3fd611a6478222e552b1230ceb717abd60cdd6df5ba30b4827511543f8f3` |
| `self-weight-engine/open_pipe_stress_self_weight_wasm_bg.wasm` | `c76e3a25727307f9503694fb7a15d440cf08eef699be1d2c437a7ab35755322a` |
| `self-weight-engine/open_pipe_stress_self_weight_wasm_bg.wasm.d.ts` | `dd78c7c8fa07894717b89a9bcda4d781200ca7a4d02a8dc153b7763290085c7b` |
| `wasm-engine/open_pipe_stress_operation_applier.d.ts` | `af2b07abf12dac915fb0e90124135d578eb7c2b473ec402dbe285900608cb8a8` |
| `wasm-engine/open_pipe_stress_operation_applier.js` | `5682432840e2199512059d7956dfdb51521179b797145a05835a5bfa0be71595` |
| `wasm-engine/open_pipe_stress_operation_applier_bg.wasm` | `d6141e2dd2431dc446232a9bc727e96fb5dc978311ea14f61f2d902be4972988` |
| `wasm-engine/open_pipe_stress_operation_applier_bg.wasm.d.ts` | `37a68fc929d21e62fad7683b94f26b7233691df567956908f345c257c8a3ae16` |

## Product source changed from the integration base

| Status | SHA-256 at candidate | Repository-relative file |
|---|---|---|
| M | `6caf90c6bd2c3a4d9a2db2ebc374fe707ded9db14a6c709332453c53013a2bb9` | `apps/desktop/e2e/gui-workflow-validation.spec.ts` |
| M | `ad14a76bdf3b87cea399d589c4f1d3fcb52affc643a88f0eb6f6ad57fbda45e0` | `apps/desktop/e2e/linear-authoring.spec.ts` |
| M | `e645e608476163bd4b35ecc1caa1d7f3fc794cc37f1dc39775639849b8aadeca` | `apps/desktop/e2e/r2-smoke.spec.ts` |
| M | `0faca6572f26354ba0ce2b907556a604d8726d483fd297bd0a5d2ef4b54fe082` | `apps/desktop/e2e/toolkit-batch-dist.spec.ts` |
| M | `d46fc3472610408cd808b22a7a93c3a073436913c4d14bf5bf1db81135fe2473` | `apps/desktop/e2e/ui-foundation-dist.spec.ts` |
| M | `a2b6fd7415e2f5de95927f8084c320dc1ea05c12549edd2e2b5d64e0da20ade2` | `apps/desktop/e2e/ui-foundation-workflows.ts` |
| M | `1b68f7d8b11297096bd119b1f88b0f9efa19fec0b79831cd78bffd11b47822b9` | `apps/desktop/e2e/ui-foundation.spec.ts` |
| M | `ed29de65b7121e1ef6a78348b10144473163d33a0a4010978a986a1fcc5a01aa` | `apps/desktop/e2e/wasm-engine-dist.spec.ts` |
| M | `56eb8bbec5b0895112bb5e6f38be3e6c5e6a544bb429f109157bbc18e6a79ac5` | `apps/desktop/e2e/workspace-driver.ts` |
| M | `b93ce5033c650265539d6ba91eb0929d9e236dd4385aabb447a94a2149ec1d97` | `apps/desktop/e2e/workspace-layout.spec.ts` |
| M | `cfa35ee767769e0cde5f9bcc6eac544c4701fa890ab2999ef6d8db0ca395fa56` | `apps/desktop/src-tauri/src/lib.rs` |
| M | `b3de0965f205e1ed34f07744939314c9ac40dd00af2ee8c0de336c6e2bfba5d0` | `apps/desktop/src-tauri/tauri.conf.json` |
| M | `58d5d3e219372d02f0c0322d2bce5f3884e32062a85d2ac62369b50e38a65e96` | `apps/desktop/src/App.deadControls.test.tsx` |
| M | `257aea975a63c9c4e52351b5fd269a0dd99acef03eb20d4f326ae7def72cf0ac` | `apps/desktop/src/App.projectHandlers.test.tsx` |
| A | `c956b2cda86b5b7586e8e7cce1a06d11ff112b707a3dbec0b95223501d7da9c6` | `apps/desktop/src/App.shell.test.tsx` |
| M | `8e5dc1622c9379302eaf29323baf8f393c5ecd6abccbc4466957b4ec9b79fad1` | `apps/desktop/src/App.test.tsx` |
| M | `0c11c7a3f38ab7be9a9719548dbb7363fa4ffd7f90a110a48a4a0b307a7746bd` | `apps/desktop/src/App.tsx` |
| M | `4bbdc032a715c1744da13ef7ed2c10afaae0bc989cd3299554af02239b560aea` | `apps/desktop/src/features/display-units/index.tsx` |
| M | `c84690b02392af87a54702453317cf9f828e358b1ec8ebb5bdbea02c5a7f6592` | `apps/desktop/src/features/toolkit/ToolkitPalette.test.tsx` |
| M | `793fcba9dfd4eadc925b62a73fa36796e59cef64daa6626b0adde41e576d3490` | `apps/desktop/src/features/toolkit/ToolkitPalette.tsx` |
| M | `85a70e3f75a9bcca9fafc7752cc7414a765915847a67c6f7c33a52d0a7d01469` | `apps/desktop/src/features/viewport/PipeViewport.tsx` |
| M | `53e143b9a8b8c6df76367fe218193f6c58e275bb6bb4f93081b5c12e6be3b30a` | `apps/desktop/src/features/viewport/viewportResource.test.ts` |
| M | `3318ae57b1c5ab3a8b77cd9df2b27df417a821afb3d8c1a362994f92150c441c` | `apps/desktop/src/features/viewport/viewportResource.ts` |
| A | `20b6f7d36e111a8b0cd6026fc1c6e76568829d46322195e03a33791295018158` | `apps/desktop/src/features/workspace/WorkspaceSessionContext.tsx` |
| D | absent; base `d6a2a818496b7e2b3fc4885eb20d4552809dddbe274b6917dcaac3506e33a37b` | `apps/desktop/src/features/workspace/WorkspaceToolbar.test.tsx` |
| D | absent; base `211844849da4ba2257d95ae542852a74360377095efaaee1259c7d054ed8433e` | `apps/desktop/src/features/workspace/WorkspaceToolbar.tsx` |
| M | `1822ee1ba7ac4271c4ad89faf30fc83f6fc3cc8d1e56a0e98d59507d8e6669bb` | `apps/desktop/src/features/workspace/chromeSessionState.ts` |
| M | `cdcd87f270dd2109084df6225d0ec14e463388405e2483b5416265832722555d` | `apps/desktop/src/features/workspace/menuCommands.ts` |
| A | `abc9b2983f3d587694ef0ff944d8b790e1e8af7158994d20ebd2f68bf9825515` | `apps/desktop/src/features/workspace/sessionBoundary.test.ts` |
| A | `fbd9b3ed429f180afa364a098d0f51ae02239e2b37d1ae1552cb26e18e9f5e3b` | `apps/desktop/src/features/workspace/shell/AgentStrip.tsx` |
| A | `9fd86feefc50904243920067186a7d4986cb7204ac5f442e7d9890c5b65c4068` | `apps/desktop/src/features/workspace/shell/DisabledReason.tsx` |
| A | `c8684af8ace795e43f6667ee682c9d1650fed653fd04ac4bef4b6c1ecb5f7074` | `apps/desktop/src/features/workspace/shell/ShellStatusBar.tsx` |
| A | `89c5ff9fabc13f75f8788a6f405bd00db8b75b4cd40d7fa794123014c5e0e268` | `apps/desktop/src/features/workspace/shell/ShellToolbar.tsx` |
| A | `1b2f2c6d937642cb7facc39573eded7829fa23b880983c89aa54aa48ccf222fd` | `apps/desktop/src/features/workspace/shell/StageRail.tsx` |
| A | `e5ecc7601c02107f54bf7f99262e7c1ae5dfff9109f6df3f87ed849113f79c88` | `apps/desktop/src/features/workspace/shell/StageTabStrip.tsx` |
| A | `292a1b082859c1a41600ae029500be117ffb4540b1a7ccf6c6ab1f2733f5509d` | `apps/desktop/src/features/workspace/shellLayout.test.ts` |
| A | `d6ea861cc2c9e6ea7a2404bcefdaf6163ab66cf74980f118809d3fa754d54b1e` | `apps/desktop/src/features/workspace/shellLayout.ts` |
| A | `857e8d7ecd1b97f0fed3fa9164138ce9bd7c15096ff952212d3a55350c379a48` | `apps/desktop/src/features/workspace/statusLabels.tokens.test.ts` |
| M | `d73e665f4efc59779acc2a60eada662c3cc2dd4ea941a7aa93424401d84ef4dd` | `apps/desktop/src/features/workspace/uiPreferences.test.ts` |
| M | `2835e94db8ddae92c5d7091a31f7ece23fffcf9537d3cc8481473f867bc2c030` | `apps/desktop/src/features/workspace/uiPreferences.ts` |
| A | `1517fc62b1709ca3b37c6e842faf07f03d5393abc3b4d5b4b04134d214d57856` | `apps/desktop/src/features/workspace/viewportPortal.test.tsx` |
| A | `b770f69d9304ffb5a685aeda7792c08b03c61129fa6da86e1364437962e782e9` | `apps/desktop/src/features/workspace/workspaceSession.shell.test.tsx` |
| M | `f7f7e6d80f137a68cbdc1ec42f5d7d87f83e551d67c091558bc1cc450a2ba28f` | `apps/desktop/src/features/workspace/workspaceSession.ts` |
| M | `03e999eacedd2e2c33ced1d56ae2e9ff6b427140052a928b8c4604c36493ff53` | `apps/desktop/src/services/nativeMenu.ts` |
| M | `cc5c177b0e73c3c9d85342e79bbbffaf18d8f0cf2094c9a2b2e36e678b3065cf` | `apps/desktop/src/styles.css` |

## Native source and lock files

| File | SHA-256 |
|---|---|
| `apps/desktop/src-tauri/src/main.rs` | `03c5b9f8fc8812114d1a7db1b1e2c5c9f2ce6c7bbf0f000565fbb872e7a1d5ca` |
| `apps/desktop/src-tauri/Cargo.toml` | `390a7a611509fce1c723d1fa198654df14f0a7651d5b18c53d0d7b04dd1ae246` |
| `apps/desktop/src-tauri/Cargo.lock` | `8b68e437515761bf5877a485cc99c52947bac386595f0182f5dcbc17732c71af` |

Raw command output is retained in `RUST_TEST_RAW.log`,
`TAURI_BUILD_RAW.log`, and `TAURI_BUILD_RAW_CONTINUATION.log`. The latter
preserves the absolute-origin lines emitted by the host build; no source file
depends on those paths.

The native UI portion of the witness is still pending owner unlock. This
manifest binds the ready final bundle even while that execution boundary is
closed.
