# PKG-00 Consolidation Manifest

Authority: piping decision `D-43` (owner instruction 2026-07-15).
Pre-consolidation basis commit: `b93764d27d0e369c1a6fe2a231b561dd71a746d2` (all removed source bytes are
recoverable from this commit; no history is rewritten).

Each member's four-document kit is consolidated into `ArchitectureBasis.md`
(`chirality-architecture-basis/v1`). At the D-43 consolidation merge,
`_STATUS.md`, `MEMORY.md`, `_SEMANTIC*`, `_CONTEXT.md`, and `_run_records/`
were byte-identical to the basis commit, while `_DEPENDENCIES.md` and
`_REFERENCES.md` changed only their SOFTWARE_DECOMP revision pins (0.7 → 0.9
per SCA-006). The later alignment recorded below changes current navigation
surfaces without rewriting the preserved semantic, memory, status, or run
evidence.

## Current reading contract

For current use of a PKG-00 member, read these surfaces in order:

1. `ArchitectureBasis.md` — current member reference and applicable
   architecture requirements, decisions, artifacts, and open questions.
2. `_STATUS.md` — sole deliverable-local lifecycle authority.
3. `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9 — current
   decomposition and AB-00-01..08 authority.
4. `_DEPENDENCIES.md` and `_REFERENCES.md` — current relationship and source
   navigation.

`_CONTEXT.md` remains the sealed revision-0.7 setup context and carries a
supersession notice. `MEMORY.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and
`_run_records/` remain dated historical or derivative evidence; their
references to the former kit do not override current state. The original
byte-identical D-43 consolidation state remains bound at merge commit
`291301291bdaa8ec6b64dbbf3cff98ecfeadeca7`.

Validate this contract with:

```text
python3 tools/validation/validate_architecture_basis.py
```

## Removed source files (SHA-256 at basis commit)

| Member | File | SHA-256 |
|---|---|---|
| DEL-00-01 | Datasheet.md | `a22a9266980ebc21e196b4dd804c89ba0a1ab32365dd581f48330946b7a3da51` |
| DEL-00-01 | Specification.md | `93ba9ec98d34d103a7c4d5a1e12cadc988d5638279d34bd8b8851006685ec5c7` |
| DEL-00-01 | Procedure.md | `a05dde59edddf69cbc8019b04558888fab8cc4e0588a04825cef794221173435` |
| DEL-00-01 | Guidance.md | `25c32b04104c7ecb72fac3a6122ec3cee39fbc07f60df5b1f2e6e3f23a1f5a39` |
| DEL-00-02 | Datasheet.md | `012740652373932f028076429ec66e24a2a45818cdfaeff349f35cc3bfe7eb32` |
| DEL-00-02 | Specification.md | `b632ce58819cd018810fd6aa8a9834b72d3de941c980ab8cdda4fa5e46b5350c` |
| DEL-00-02 | Procedure.md | `bc78c339ab16c4af41021e8d980e751a492cd8bc1a6e3448e7557205556ee690` |
| DEL-00-02 | Guidance.md | `f03eef0506a26d5e24f52afa1f178e50979ad6c8e68229d500ca918e15b387df` |
| DEL-00-03 | Datasheet.md | `40b0c04ad4e1422d7982582cef5eb8909cb03250ae5fb07da1463f212e2f8794` |
| DEL-00-03 | Specification.md | `7686196a49c08bbda3c42f64a148a05b084ccdedc9d00a61565fb243ae2ef4d9` |
| DEL-00-03 | Procedure.md | `a75c0a60a43a6c48ed55b57ecf23b3745caf6dce4e7e8c3ae2934e751e6a1729` |
| DEL-00-03 | Guidance.md | `0e6ee8b03eb56a0c9d43257cedf6794c2f9725d94208df972703bc84d03912bc` |
| DEL-00-04 | Datasheet.md | `4353d4ffbce4beb2dd51cd230d1f639095668a7a573498f3c81262bcf37d2bb6` |
| DEL-00-04 | Specification.md | `0475503ba2509d3501eb154796d569b41a7ffe8f5783ca25a1be4f1f1008a7d3` |
| DEL-00-04 | Procedure.md | `f2219125098c4f201f8b28e5e48721284bcff6a16eeebf70c7f39544b786afe1` |
| DEL-00-04 | Guidance.md | `5577655d54e556b53517132b51783173d828ec38b842c1335e319f0f666accbf` |
| DEL-00-05 | Datasheet.md | `ea6fc4595cf5d46e1dfbf2fd3c2f2146277b67fe9fa46afc03bec6ec68619c23` |
| DEL-00-05 | Specification.md | `fe0a7ee308309c8af26855b85ee40552e3b487c04f6367d4b5ff2bc1d222775e` |
| DEL-00-05 | Procedure.md | `600a1b116e5caaa9becd0491a7daf9fd291102e126dff72ec2d5ec05fb841b77` |
| DEL-00-05 | Guidance.md | `816127ee3f58c0a9f2a813305a197cade9f42fe5ff59fdc61f54f7e26379dfa1` |
| DEL-00-06 | Datasheet.md | `1ba1d2986e6421d5f013dbd6e7a63dedc3ead1a0d011be92149d65a8e8c4cf15` |
| DEL-00-06 | Specification.md | `632365472f693a5dace374c33d70f2efc7377662d31358ddb16977cc1232ce8e` |
| DEL-00-06 | Procedure.md | `4b9d3e7aded46ca00d8974f41d5efaba46da98a9d340c2a94b5f01fcbd34ca6a` |
| DEL-00-06 | Guidance.md | `2a43bc248ee2b3ec24f268493d8b2690c465a3513dfaad649436c7085754eb4c` |
| DEL-00-07 | Datasheet.md | `8338c157fcb27e9f14d6bfa4d9ea64589cea3e994e4310ad466f8fa4dde4cd98` |
| DEL-00-07 | Specification.md | `8637675273f3cc1b18561dc2ecb822f8444bed6fba04c35e42cba47ca5f1f90c` |
| DEL-00-07 | Procedure.md | `92034dec7f63dd1b06e10cd8ec377921ed5db4f1c59ea75e0e9ef61828a292b7` |
| DEL-00-07 | Guidance.md | `2a0434dfcbd50f8b289d456d0dfd5e11155aa4bb6f554c4d8600b3e105f947b8` |
| DEL-00-08 | Datasheet.md | `2fbaceb140a420aad94e505aed0c0abb51dd9ec544404330e78b097c066f85b9` |
| DEL-00-08 | Specification.md | `e10b15e2b4d2100a16fb017f0e53c5bcff045c0b2d8468fb3336e55217377462` |
| DEL-00-08 | Procedure.md | `7427c9cac801f52ca776db0b08a38f443bc0724fa6ba7821572f1497adf85235` |
| DEL-00-08 | Guidance.md | `45b916878e2b6fac6fe2c465aebcc905e658f7b39018cf97a546738a7573c147` |

## Consolidated successors (SHA-256 as committed)

| Member | File | SHA-256 |
|---|---|---|
| DEL-00-01 | ArchitectureBasis.md | `7200e1989e45ad12b9fefcfe168d31d9fe4ab947f346437ff65a8750149aad56` |
| DEL-00-02 | ArchitectureBasis.md | `b6c54568f34a83bb54f711dc937bba68a65188041989f32f6e8ebc79737f6c5e` |
| DEL-00-03 | ArchitectureBasis.md | `8434db63ebd6388adb78cf3443f81a7501ec28c28b8d9125ea9dd63352209e08` |
| DEL-00-04 | ArchitectureBasis.md | `43e6b7f9f760ffc3769313bfb0c11507ffa482ddb9bb6573c21ba2a50de32e90` |
| DEL-00-05 | ArchitectureBasis.md | `d116c2a3529ed55bca3d53141c2f3cdf5d86f61de935f92e0494ba257b301d1d` |
| DEL-00-06 | ArchitectureBasis.md | `d0a2e8f40c010f68d5549c90dd70cbe0da3924e7b46f7252793231ec269b7827` |
| DEL-00-07 | ArchitectureBasis.md | `580bd3d2a774eac8c9476f920084b7bb461f0f18f0bc3a8d6037a0662f6c355e` |
| DEL-00-08 | ArchitectureBasis.md | `6880b11a9c74acff8fbaa1c0d0536c6d7d85ba3f2f0da1314cf081821f238553` |

Preserved-source citations to the removed kit paths inside converted
`ScopeOfWork.md` documents and `_SEMANTIC*` control files are historical
references bound by the Stage-2 preservation gates; they resolve via the
basis commit above. The live successor for every cited kit surface is the
member's `ArchitectureBasis.md`.
