# The 14 formation rows (GATE/FORMATION_EXCEPTIONS.json, db665f2cb; ruling text amended at 42b300344, rows unchanged), base 3488a236a against the S11-F candidate

Produced by `formation_rows.py.txt` from the probe harness outputs (harness/, one run per row: request, mode, entry). obs is the published binary64 value decoded exactly and converted exactly from its unit; ratio = |obs - exp| / (1e-9 max(|exp|, scale)), exact, with R1's binding net-governed scale.

| entry | case | key | mode | exp | obs base | ratio base | obs candidate | ratio candidate | bit-identical | no worse |
|---|---|---|---|---|---|---|---|---|---|---|
| captured | RF-CANCEL-UDL-W1e8 | th.S1.RZ | dense_scrutiny | 2.275405069889944947747441610685845536501e-8 | 2.2754051756312433e-08 | 46.4714 | 2.2754051790793294e-08 | 47.9868 | no | no |
| captured | RF-CANCEL-UDL-W1e8 | th.S1.RZ | sparse_interactive | 2.275405069889944947747441610685845536501e-8 | 2.2754051756312433e-08 | 46.4714 | 2.2754051790793294e-08 | 47.9868 | no | no |
| typed | RF-CANCEL-F-G1e80-GnG-INPLANE | Mb.M1.j | dense_scrutiny | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, 4.263256414560601e-14) | 9.99996e+08 | hypot(0.0, 1.0000036354540498e-08) | 3635.45 | no | yes |
| typed | RF-CANCEL-F-G1e80-GnG-INPLANE | Mb.M1.j | sparse_interactive | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, 2.842170943040401e-14) | 9.99997e+08 | hypot(0.0, 1.0000022143685783e-08) | 2214.37 | no | yes |
| typed | RF-CANCEL-F-G1e80-GnG-INPLANE | Mb.M2.i | dense_scrutiny | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, -4.973799150320701e-14) | 9.99995e+08 | hypot(0.0, -9.999993721976352e-09) | 627.802 | no | yes |
| typed | RF-CANCEL-F-G1e80-GnG-INPLANE | Mb.M2.i | sparse_interactive | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, -2.1316282072803006e-14) | 9.99998e+08 | hypot(0.0, -1.0000043459967856e-08) | 4346 | no | yes |
| typed | RF-CANCEL-M-G1e80-GnG-INPLANE | Mb.M2.i | dense_scrutiny | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, -1.4210854715202004e-14) | 9.99999e+08 | hypot(0.0, -1.000005767082257e-08) | 5767.08 | no | yes |
| typed | RF-CANCEL-M-G1e80-GnG-INPLANE | Mb.M2.i | sparse_interactive | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, -3.552713678800501e-14) | 9.99996e+08 | hypot(0.0, -1.0000043459967856e-08) | 4346 | no | yes |
| typed | RF-CANCEL-M-G1e80-GnG-INPLANE | Mb.M2.j | dense_scrutiny | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, 2.842170943040401e-14) | 9.99997e+08 | hypot(0.0, 9.999993721976352e-09) | 627.802 | no | yes |
| typed | RF-CANCEL-M-G1e80-GnG-INPLANE | Mb.M2.j | sparse_interactive | 1.000000000000000000000000000000000000000e-8 | hypot(0.0, -1.4210854715202004e-14) | 9.99999e+08 | hypot(0.0, 9.999993721976352e-09) | 627.802 | no | yes |
| typed | RF-CANCEL-UDL-W1e8 | th.S1.RZ | dense_scrutiny | 2.275405069889944947747441610685845536501e-8 | 2.2754051756312433e-08 | 46.4714 | 2.2754051790793294e-08 | 47.9868 | no | no |
| typed | RF-CANCEL-UDL-W1e8 | th.S1.RZ | sparse_interactive | 2.275405069889944947747441610685845536501e-8 | 2.2754051756312433e-08 | 46.4714 | 2.2754051790793294e-08 | 47.9868 | no | no |
| typed | RF-CANCEL-UDL-W1e80 | th.S1.RZ | dense_scrutiny | 4.627942515030396503893101581055957023392e-16 | 1.2184480799204924e+57 | 2.63281e+81 | 1.2184480799204924e+57 | 2.63281e+81 | yes | yes |
| typed | RF-CANCEL-UDL-W1e80 | th.S1.RZ | sparse_interactive | 4.627942515030396503893101581055957023392e-16 | 1.2184480799204922e+57 | 2.63281e+81 | 1.2184480799204922e+57 | 2.63281e+81 | yes | yes |

The four UDL-W1e8 rows are 3% worse (46.47 to 47.99): S11-F publishes the correctly rounded net of the represented terms (0.2, -33333333.333333325, +33333333.625000015) = 0.4916666902601719, whose formation error (+2.36e-8 against the intended 0.4916666666666667) the base fold happened to reduce. ROOT accepted this on the formation-class condition (published value = correctly rounded net of the represented terms; row stays pinned; S11-G owns it). UDL-W1e80 is bit-identical; the four INPLANE triples improve from about 1e9 to 628-5767.

The candidate values in this table are exactly pinned by `f1_f11_f12_rf_cancel_cases_meet_the_binding_predicate_on_both_entries` (`FORMATION_PINS` in `core/product_physics/src/s11f_tests.rs`), per ROOT's amended formation-class condition (`T3/ROOT_RULINGS_V1.md`, 42b300344): any change to a formation row's published bits fails that test.
