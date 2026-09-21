**PASS — complete 2-path backcheck** of `6dd0e466f4432f21d3ce6913782b95f9a325f700..1ad964e30b08bed57bba8f08e3a75e8c883c8b0b`. No actionable findings.

The explicit shrinking grid track confines content to the available width. The coordinate rowgroup shares the existing 420 px row minimum, so header and body scroll together inside the coordinate grid. Footer and family controls remain outside that scroller. Existing bulk-family scrolling remains available; outer pane sizing rules and protected thresholds are unchanged.

Recorded measurements support the diagnosis and correction: before repair, the grid could not scroll and ModelTree moved 84 px; maintained CSS gives the grid 96 px of horizontal travel while ModelTree and the filter remain stationary. Header/body widths and positions agree.

Verified **12 unique browser passes**: fit 2, prior B4 4, legacy 6. The final fit-only repeat passed 2/2 in 5.2 s after adding y/height assertions; it adds no unique coverage. Original assertions remain intact. All **20 freeze07 hashes match committed bytes**, and exactly the two authorized paths changed.

The rectangle assertions establish stability of the table pane and **viewport host DIV during scrolling**. They establish neither drawn WebGL dimensions nor D-72 qualification. Actual native `1ad964e3` pointer-fit verification, final sweep, combined review and actual-head CI remain pending. Prior native Cancel/Apply evidence remains bound to `6dd0e46`.

Same retained independent TASK, Codex/Astra xhigh; no delegation, execution, UI or mutations. New context/evidence hashes:

| Record | SHA256 |
|---|---|
| ROOT fit backcheck brief | `dfdaecb9ce5cfc463851abe2a6068525ba55f8d97edb90710c801c57098437ab` |
| `SOURCE_FREEZE_07.json` | `bdcbafa9eafe7479d5ea507994ff87d18b6e0b17c2c8f3ddcc622c682346de43` |
| `WORKER_RETURN_FREEZE07.md` | `2cdc92e4b82337e37cb716b71945c507ef86073dc0701a190117085459f86d1a` |
| Fit diagnosis/checks | `82a88ae5609ffd07d76b7ac157b51ef503822e81de60be98c0bbd014b7fe09e9` |
| Before measurements | `afb411d7ff2c45dffc886c45c2b21cbdae098a8981cc4281842975671b121b8b` |
| Maintained measurements | `d462534263ef7f6ce0b582ee5d442fecf09fe4bfd46f6ed48ed0f40fa55a6bec` |
| Final fit log | `fec58f8c5551b0ce9ead824a2925fa31d0543f219777dbb362324544bdb5d03c` |
| Prior B4 log | `f673aa098db56929e41a63daccd0be44d8e2241fb7c9dc0cee4fe4982f02551a` |
| Legacy six log | `2d943bbf3aa95cc167bd8e0770b2931fa8b8196e047b84a0b04e2a476b3af1d2` |
