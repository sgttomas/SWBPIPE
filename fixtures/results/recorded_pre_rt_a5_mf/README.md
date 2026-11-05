# Table variants before RT-A5 `minuend_final`

Main's preview-physics-2, preview-physics-timoshenko-1 and preview-physics-retained-1 tables at
b32a17761 (#107; identical at 400a409d7), verbatim (`git show b32a17761:fixtures/results/<name>.json`),
named by their SHA-256 prefix. `minuend_final` restated the preview-physics combination policy
(`PREVIEW-PHYSICS-1-COMBINATION-v3` → `-v4`) and added the `held_status_operand` basis; rows and
limitations did not move. Members of the closed recorded-table registry (Rust
`recorded_tables::VARIANTS`, Python `compatibility.RECORDED_PRE_RT_A4_TABLES`, desktop
`recordedPreRtA4.ts`). History: never repinned or edited.
