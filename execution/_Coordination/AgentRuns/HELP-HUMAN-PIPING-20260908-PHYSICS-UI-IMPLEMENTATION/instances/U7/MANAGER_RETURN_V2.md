# U7 Successor Manager Return V2

Status: `PASS_FOR_SAME_RU_BACKCHECK`; native and integration gates remain held.

R2 repaired exactly terminal `RU-F1` through `RU-F5` inside the original nine-path fence. Busy route mutation now stays disabled through Add/Apply and permitted selection invalidation cancels App-owned publication. Single and batch Apply results are fully bound to the frozen submission, basis, exact diff/steps, complete acceptance, validation, diagnostics, and recomputed applied/batch hashes before any state setter. Continue survives the component's own accepted commit, runtime reservations require a complete discriminator and reject intra-route ID reuse, and node/pipe/pointer provenance starts blank until explicit entry.

Frozen successor bindings:

- `SUCCESSOR_MANIFEST_V2.json`: SHA-256 `72d1cb5f31d7d81e92f05edd6f0b9e8b74c102cc27c898b7139e99644fb529f1`;
- full base-to-successor diff: SHA-256 `8c01ab323ef27836ecb1e777366b9166adfe28f9948bd990b01441fc2302f30e`, 3,025 lines, nine paths;
- byte-exact V1-to-successor delta: SHA-256 `09881c4a563bd919a52bd855fc954c798565be9d8817dabfbf36a32793f83863`, 1,102 lines, six paths;
- five-finding closure map: SHA-256 `5601018513ac2d61cdcc59796e14154e290861254658632022f69d63f8424528`;
- updated seven-row consumer proof: SHA-256 `324acb1aeeff8db2bba679ce55dd41f262561c5bc7f5dff242535971feeb840b`;
- R2 child return: SHA-256 `0aef9be19b342f376266672df88481f45641d7111287fd6588125ef01fe51af9`;
- manager validation: SHA-256 `dbed0f9b0e4ff7e586a27daf904cb5f5da6757a672a8441f318e091b0eda4175`.

Final checks: route 17/17 PASS; App 157/157 PASS; inspector 9/9 PASS; desktop `tsc -b && vite build` PASS; Chromium 1/1 PASS at 1024×768. The final delta after the 157-test App run consists only of TypeScript non-null assertions at values already guarded by `applyResultMatchesSubmission`; the assertions erase from emitted JavaScript. The affected App set and final build/browser passed after that narrowing repair. The initial build failure and all six explicit-provenance fixture failures are preserved in the child return.

Manager rehashed all live source and evidence members, reconstructed V1 from base plus its frozen exact diff with all nine V1 hashes matching, and generated the portable full and delta diffs. Containment, CR, trailing whitespace, final LF, and scoped diff checks pass. No manager product edit or test rerun occurred.

Same RU may now backcheck the successor against its terminal findings. The original V1 manifest/diff/proofs, RU originals, and historical screenshot remain immutable. Consumer proof does not declare formal dependency-row acceptance. Packaged native proof, root fan-in, commit, and push remain pending.
