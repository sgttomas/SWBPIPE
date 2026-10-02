# B reconciliation and per-case results

All 16 B cases remain selected. Their published numeric bits, value/range outcomes and row classes are unchanged. Source/policy identities, total work, and 12 precision/floor/bound records materially change; output bytes are not unchanged. Full keyed changes, old/new raw hashes and basis are in B_BEFORE_AFTER.json. Old source was 3bddc2b05f6106e969c7cf43373b230845c7cc66, policy v1; new source is dd1f70d8ba85b19f7d948bca6ee08a44bbb12ae1, policy v2.

| Case | Old → new p | Old → new LME | Floor / bound change |
|---|---:|---:|---|
| B01 | 128 → 512 | 1,088,017 → 5,764,851 | Force scale 0→h; 12 b: 0→2h; both F/M floors h |
| B02 | 256 → 512 | 2,015,063 → 4,876,556 | Moment scale 0→h; 12 b: 0→2h; both F/M floors h |
| B03 | 128 → 128 | 999,009 → 1,847,194 | None |
| B04 | 128 → 128 | 1,052,981 → 1,919,087 | None |
| B05 | 128 → 512 | 1,087,693 → 5,869,317 | Force scale 0→h; 12 b: 0→2h; both F/M floors h |
| B06 | 128 → 512 | 1,087,693 → 5,763,867 | Force scale 0→h; 12 b: 0→2h; both F/M floors h |
| B07 | 256 → 512 | 2,014,975 → 4,876,660 | Moment scale 0→h; 12 b: 0→2h; both F/M floors h |
| B08 | 256 → 512 | 2,015,198 → 4,876,998 | Moment scale 0→h; 12 b: 0→2h; both F/M floors h |
| B09 | 128 → 128 | 999,320 → 1,600,806 | None |
| B10 | 128 → 128 | 1,088,785 → 1,744,573 | None |
| B11 | 128 → 512 | 1,088,044 → 5,764,987 | Force scale 0→h; 12 b: 0→2h; both F/M floors h |
| B12 | 256 → 512 | 2,012,982 → 4,872,178 | Moment scale 0→h; 12 b: 0→2h; both F/M floors h |
| B13 | 128 → 512 | 1,088,101 → 5,765,026 | Force scale 0→h; 12 b: 0→2h; both F/M floors h |
| B14 | 256 → 512 | 2,012,982 → 4,871,962 | Moment scale 0→h; 12 b: 0→2h; both F/M floors h |
| B15 | 128 → 512 | 1,087,854 → 5,657,037 | Force scale 0→h; 12 b: 0→2h; both F/M floors h |
| B16 | 256 → 512 | 2,012,550 → 4,693,294 | Moment scale 0→h; 12 b: 0→2h; both F/M floors h |

Here h=2^-1074. The 12 changed cases each now select p512/P1024 with Force and Moment floor h; 144 absolute bounds change in total. Previously empty floor lists are preserved in B_BEFORE_AFTER.json. B03/B04/B09/B10 retain p128 and their scales/bounds. All costs rise, including these four. Torsion cases reject at p128 and p256 on I-end Uy Force AbsoluteBound. Short axial cases retain their old p128 ForceStopRule rejection and now reject p256 I-end Ry Moment AbsoluteBound. No new numeric claim is inferred from a rejection. B15 D9 and B16 D6/M1 retain their correct underflow outcomes.

B10's zero-rotation coverage remains input-derived only. The extra free-zero-rotation source reaches a free Rx row, but is refused at the ceiling, so it does not prove that such a row can be selected.

| Case | Outcome; p/P | LME | Compared rows | Checker exit |
|---|---|---:|---:|---:|
| C17 | Selected; 256/512 | 3404181 | 36 | 0 |
| B01 | Selected; 512/1024 | 5764851 | 37 | 0 |
| B02 | Selected; 512/1024 | 4876556 | 37 | 0 |
| B03 | Selected; 128/256 | 1847194 | 37 | 0 |
| B04 | Selected; 128/256 | 1919087 | 37 | 0 |
| B05 | Selected; 512/1024 | 5869317 | 37 | 0 |
| B06 | Selected; 512/1024 | 5763867 | 37 | 0 |
| B07 | Selected; 512/1024 | 4876660 | 37 | 0 |
| B08 | Selected; 512/1024 | 4876998 | 37 | 0 |
| B09 | Selected; 128/256 | 1600806 | 37 | 0 |
| B10 | Selected; 128/256 | 1744573 | 37 | 0 |
| B11 | Selected; 512/1024 | 5764987 | 37 | 0 |
| B12 | Selected; 512/1024 | 4872178 | 37 | 0 |
| B13 | Selected; 512/1024 | 5765026 | 37 | 0 |
| B14 | Selected; 512/1024 | 4871962 | 37 | 0 |
| B15 | Selected; 512/1024 | 5657037 | 37 | 0 |
| B16 | Selected; 512/1024 | 4693294 | 37 | 0 |
| EXTRA-FM-01 | Selected; 128/256 | 1965703 | 40 | 0 |
| EXTRA-MF-01 | Selected; 128/256 | 1769322 | 40 | 0 |
| EXTRA-ZR-01 | Unresolved(Ceiling) | 5519468 | 0 | 3 |

Every case was invoked exactly once, in this table's order, at case/invocation limits 100000000. All process exits were 0. All selected comparisons returned 0 with exact source binding. EXTRA-ZR-01 comparison returned 3, source_binding_exact=true, numeric_accuracy_pass=null and zero compared rows. It is an observed named nonselection, not an accuracy pass or a general exclusion.

The selected total is 708 rows: 705 numeric values and 3 underflows. Each full raw TSV, time/resource output, comparator stdout/stderr and unchanged comparator JSON is retained under cases/<ID>/. COMMANDS.json records dispatch, actual argv/env, guard observations, tool results and the per-case review before advance.

