Development comparison only. In any SWBPIPE result, 'checks passed', 'established' and certified bounds rest on a premise the product does not verify, which is false on a constructed model (DEL-04-01; exit criterion 18, not met). They are not established engineering results. The mechanics values are compared with CAEPIPE on that understanding.

Nothing in this pack is a SWBPIPE result. Where a case is held until the product publishes it as checks passed, that phrase names the product's own gate and carries this qualification.

# SWBPIPE sitting run sheet

Run the parts in this order. If the sitting must be shortened, strike parts from the end: part 5 first, then part 4, part 3 and part 2. Part 1 is never struck to make room for a later part. If time runs out within it, stop after a complete strike number: the lowest numbers are run first, and strike numbers 32 to 34 are the first to go.

Tick each line when you have run its case, or written on its run line why not.

## Part 0: Dry run

Three small first-tranche cases taken through the batch route one line at a time, with your own comparison of the first print's section titles with the short list, and three copies of one dynamics run file: one run by the batch command, one opened in the program's window for the observations the benchmark manifest lists, and a sacrificial duplicate on which the export is tried (check (v)). By the benchmark manifest's consequences, its answers decide how parts 1 and 4 are run.

Evidence of the model run: for the batch lines, whatever the program writes beside each model file, with the print; for the window sample, the model saved as `sample-dyn-window.mod` (INSTRUCTIONS.md section 2).

Batch lines: group 1 of `BATCH_COMMANDS.txt` after the two set-up lines, 4 line(s), pasted one line at a time in this order.

Write down what you observe, after each colon. Nothing here says what you will see. Write the answers to C and to checks (i) to (iv) also on the run line of `sample-dyn-window`, and the answers to check (v) on the run line of `sample-dyn-export-trial`.

- A. the dry run's first case line, pasted alone: what the Command Prompt and the program did, and the files the line left in its case's folder, by name: 
- B. the batch sample (sample-dyn-batch), whose file carries the dynamics options record (OPMISC, with a number of modes) and the spectrum table: whether its print lists modes, and the section titles it holds: 
- C. after the import in the window sample's folder, whether the import wrote a model file, and where (the export page, CAEPIPE-EXPORT-MBF, says "But note that the existing .mod file will be overwritten. So, you must rename the edited text file before importing it back into CAEPIPE." and does not say in which folder): 
- (i) compare the spectrum table the program shows with the stated values, and note agree or not: 
- (ii) compare the Dynamics tab's Cutoff Frequency and Number of Modes with the stated values, and note agree or not; if the window shows the file's value in another unit, note that once: 
- (iii) write down the command that saved the imported model; whether the program let you choose the name and the folder; the name and folder the saved file took; whether the program asked anything: 
- (iv) note what the spectrum options window (Options > Spectrum) offers for axis units and interpolation: 

Check (v) is made in `samples/sample-dyn-export-trial` alone. From the benchmark manifest:

(v) on a sacrificial duplicate: the folder samples/sample-dyn-export-trial holds sample-dyn-export-trial.mbf, a copy of the same run file as the window sample (cpb-dyn-c-n01-flat--srss-srss); in that folder only:
1. Import sample-dyn-export-trial.mbf: File > Import from the Main window, select the file and click Import. [https://docs.sstcae.com/users_manual/import_mbf.htm ("IMPORT MBF"; CAEPIPE-IMPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
2. From the Layout window, choose File > Export ("To export a model, select the menu command File > Export from the Layout window."). [https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, "EXPORT MBF"; CAEPIPE-EXPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
3. Before touching the dialog, write down the files the folder now holds, by name.
4. In the Export Model dialog, choose "Model batch file (*.mbf)" as the Save as type. [https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, "EXPORT MBF"; CAEPIPE-EXPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
5. Note whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, and whether it displays them before anything is written.
6. If it does, click Export; if not, click Cancel. [https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, "EXPORT MBF"; CAEPIPE-EXPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
7. Note the files the folder then holds, by name.
8. Nothing in samples/sample-dyn-window or in any case folder is opened or touched by this trial.

- (v) step 3, before touching the dialog, the files the folder now holds, by name: 
- (v) step 5, whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, and whether it displays them before anything is written: 
- (v) step 6, Export or Cancel ("If it does, click Export; if not, click Cancel"): 
- (v) step 7, the files the folder then holds, by name: 

The benchmark manifest's consequences of the checks (i), (ii), (iii) and (v):

- (i) not agreeing, or not made: form B (spectrum table typed) for every dynamics run
- (ii) another unit: the cutoff correction applies to whichever single form of a pair is run; the owner sets the stated value only if a public page states the conversion, and otherwise skips the run and notes it
- (iii) the model cannot be saved as <run id>.mod in the run's folder: every hand-entry and every dynamics run is skipped and noted
- (v) the dialog did not let you set the name and the folder, or a file was written before a destination was offered: no run exports at this sitting, and the export steps of every run are skipped and noted

- the consequences that apply (write their numbers, or none): 

| # | item | kind | layout rows | MBF SHA-256 (first 12) | folder | done |
|---|---|---|---|---|---|---|
| 1 | cpb-g1-05 | benchmark case | 4 | 22b19ece73eb | `cases/cpb-g1-05` | [ ] |
| 2 | cpb-g8-01 | benchmark case | 4 | 6094e1cff503 | `cases/cpb-g8-01` | [ ] |
| 3 | cpb-g6-02 | benchmark case | 7 | 9a3805aa5e48 | `cases/cpb-g6-02` | [ ] |
| 4 | sample-dyn-batch | dry-run sample, batch | 3 | 239c4758a55c | `samples/sample-dyn-batch` | [ ] |
| 5 | sample-dyn-window | dry-run sample, window | 3 | 239c4758a55c | `samples/sample-dyn-window` | [ ] |
| 6 | sample-dyn-export-trial | dry-run sample, export trial | 3 | 239c4758a55c | `samples/sample-dyn-export-trial` | [ ] |

## Part 1: Dynamics runs

The modal and response-spectrum runs of tranche 3, in the benchmark manifest's strike order. Each pair has two forms, each in its own folder, and one case is declared in one form only; every case of this part is run in the program's window only.

Evidence of the model run: the model saved as `<run id>.mod` from inside the program before the analysis, brought back in the run's folder with its print (INSTRUCTIONS.md section 2).

Window only: no case of this part has a line in `BATCH_COMMANDS.txt`.

From the benchmark manifest: "Exactly one of a pair is run at the sitting: form A only if the dry run's check of the dry-run sample that carries the same spectrum table agreed, form B otherwise."

From the benchmark manifest: "cpb-dyn-c-n08-two is declared in the typed form only (series dynamics-comparable-table-typed, run cpb-dyn-c-n08-two--srss-srss-table-typed). Its spectrum table (DYNTWO, three rows) is carried by no dry-run sample, so the condition on which the form that writes the table may be run could never be met; by design the typed form is this case's only form. It is not a run that was skipped"

A window instruction may end with a paragraph offering, on a condition it states, to make a run's setting in the model of an earlier run of the same case that is still open, instead of importing the run's file. Where its condition holds and you take it, save the model and print the results under that run's name in that run's folder, as the paragraph says, and write on its run line that the setting was made by hand. The files expected back in that folder are the same three.

Tick the form you ran, and write "not run: other form" on the other form's run line. This part is never struck to make room for a later part. If time runs out within it, stop after a complete strike number: the lowest numbers are run first, and strike numbers 32 to 34 are the first to go.

| # | strike order | form | item | layout rows | MBF SHA-256 (first 12) | folder | done |
|---|---|---|---|---|---|---|---|
| 7 | 1 | A, table added | cpb-dyn-c-n08-flat--srss-srss | 10 | c0ba26eadf2e | `cases/cpb-dyn-c-n08-flat--srss-srss` | [ ] |
| 8 | 1 | B, table typed | cpb-dyn-c-n08-flat--srss-srss-table-typed | 10 | aca7eea33e77 | `cases/cpb-dyn-c-n08-flat--srss-srss-table-typed` | [ ] |
| 9 | 2 | A, table added | cpb-dyn-c-n02-flat--srss-srss | 4 | 190ea04deb2e | `cases/cpb-dyn-c-n02-flat--srss-srss` | [ ] |
| 10 | 2 | B, table typed | cpb-dyn-c-n02-flat--srss-srss-table-typed | 4 | d29393911f27 | `cases/cpb-dyn-c-n02-flat--srss-srss-table-typed` | [ ] |
| 11 | 3 | A, table added | cpb-dyn-c-n16-flat--srss-srss | 18 | c75f977d57ef | `cases/cpb-dyn-c-n16-flat--srss-srss` | [ ] |
| 12 | 3 | B, table typed | cpb-dyn-c-n16-flat--srss-srss-table-typed | 18 | 4b9cb25fd475 | `cases/cpb-dyn-c-n16-flat--srss-srss-table-typed` | [ ] |
| 13 | 4 | A, table added | cpb-dyn-c-n08-trunc--srss-srss | 10 | a62764be8438 | `cases/cpb-dyn-c-n08-trunc--srss-srss` | [ ] |
| 14 | 4 | B, table typed | cpb-dyn-c-n08-trunc--srss-srss-table-typed | 10 | 91a6005d315c | `cases/cpb-dyn-c-n08-trunc--srss-srss-table-typed` | [ ] |
| 15 | 5 | A, table added | cpb-dyn-d-n08-flat--srss-srss | 10 | 72a336cf8d99 | `cases/cpb-dyn-d-n08-flat--srss-srss` | [ ] |
| 16 | 5 | B, table typed | cpb-dyn-d-n08-flat--srss-srss-table-typed | 10 | c381cbe6ad07 | `cases/cpb-dyn-d-n08-flat--srss-srss-table-typed` | [ ] |
| 17 | 6 | only form | cpb-dyn-c-n08-two--srss-srss-table-typed | 10 | 9d9ac4d22e71 | `cases/cpb-dyn-c-n08-two--srss-srss-table-typed` | [ ] |
| 18 | 7 | A, table added | cpb-dyn-m-flat--srss-srss | 13 | da83f9c56190 | `cases/cpb-dyn-m-flat--srss-srss` | [ ] |
| 19 | 7 | B, table typed | cpb-dyn-m-flat--srss-srss-table-typed | 13 | 68c8df47c5b7 | `cases/cpb-dyn-m-flat--srss-srss-table-typed` | [ ] |
| 20 | 8 | A, table added | cpb-dyn-c-n04-flat--srss-srss | 6 | 67107e13404a | `cases/cpb-dyn-c-n04-flat--srss-srss` | [ ] |
| 21 | 8 | B, table typed | cpb-dyn-c-n04-flat--srss-srss-table-typed | 6 | f20f5a3f5d83 | `cases/cpb-dyn-c-n04-flat--srss-srss-table-typed` | [ ] |
| 22 | 9 | A, table added | cpb-dyn-l-flat--srss-srss | 9 | 66a7197b64df | `cases/cpb-dyn-l-flat--srss-srss` | [ ] |
| 23 | 9 | B, table typed | cpb-dyn-l-flat--srss-srss-table-typed | 9 | 57e8aff66d03 | `cases/cpb-dyn-l-flat--srss-srss-table-typed` | [ ] |
| 24 | 10 | A, table added | cpb-dyn-c-n01-flat--srss-srss | 3 | 239c4758a55c | `cases/cpb-dyn-c-n01-flat--srss-srss` | [ ] |
| 25 | 10 | B, table typed | cpb-dyn-c-n01-flat--srss-srss-table-typed | 3 | ff88c07bc5d1 | `cases/cpb-dyn-c-n01-flat--srss-srss-table-typed` | [ ] |
| 26 | 11 | A, table added | cpb-dyn-c-n08-flat--abs-abs | 10 | 0bce3de1125b | `cases/cpb-dyn-c-n08-flat--abs-abs` | [ ] |
| 27 | 11 | B, table typed | cpb-dyn-c-n08-flat--abs-abs-table-typed | 10 | 410df72cfcf8 | `cases/cpb-dyn-c-n08-flat--abs-abs-table-typed` | [ ] |
| 28 | 12 | A, table added | cpb-dyn-c-n08-flat--abs-srss | 10 | 3a6d1da729bd | `cases/cpb-dyn-c-n08-flat--abs-srss` | [ ] |
| 29 | 12 | B, table typed | cpb-dyn-c-n08-flat--abs-srss-table-typed | 10 | 3ed7047481f8 | `cases/cpb-dyn-c-n08-flat--abs-srss-table-typed` | [ ] |
| 30 | 13 | A, table added | cpb-dyn-c-n08-flat--srss-abs | 10 | 0f4fcf3baa95 | `cases/cpb-dyn-c-n08-flat--srss-abs` | [ ] |
| 31 | 13 | B, table typed | cpb-dyn-c-n08-flat--srss-abs-table-typed | 10 | a4746ad8f51d | `cases/cpb-dyn-c-n08-flat--srss-abs-table-typed` | [ ] |
| 32 | 14 | A, table added | cpb-dyn-c-n08-flat--closely-spaced-abs | 10 | c93d472ef7cc | `cases/cpb-dyn-c-n08-flat--closely-spaced-abs` | [ ] |
| 33 | 14 | B, table typed | cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed | 10 | a71a1f290090 | `cases/cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed` | [ ] |
| 34 | 15 | A, table added | cpb-dyn-c-n08-flat--closely-spaced-srss | 10 | 09b89527804f | `cases/cpb-dyn-c-n08-flat--closely-spaced-srss` | [ ] |
| 35 | 15 | B, table typed | cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed | 10 | e051265774bb | `cases/cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed` | [ ] |
| 36 | 16 | A, table added | cpb-dyn-c-n08-flat--nrl-abs | 10 | 92d8575a66a9 | `cases/cpb-dyn-c-n08-flat--nrl-abs` | [ ] |
| 37 | 16 | B, table typed | cpb-dyn-c-n08-flat--nrl-abs-table-typed | 10 | 53d3ab580a4b | `cases/cpb-dyn-c-n08-flat--nrl-abs-table-typed` | [ ] |
| 38 | 17 | A, table added | cpb-dyn-c-n08-flat--nrl-srss | 10 | 51171580a552 | `cases/cpb-dyn-c-n08-flat--nrl-srss` | [ ] |
| 39 | 17 | B, table typed | cpb-dyn-c-n08-flat--nrl-srss-table-typed | 10 | 80628c8b686d | `cases/cpb-dyn-c-n08-flat--nrl-srss-table-typed` | [ ] |
| 40 | 18 | A, table added | cpb-dyn-c-n02-flat--abs-abs | 4 | 02c1f9771c80 | `cases/cpb-dyn-c-n02-flat--abs-abs` | [ ] |
| 41 | 18 | B, table typed | cpb-dyn-c-n02-flat--abs-abs-table-typed | 4 | 461f1fe13b95 | `cases/cpb-dyn-c-n02-flat--abs-abs-table-typed` | [ ] |
| 42 | 19 | A, table added | cpb-dyn-c-n02-flat--abs-srss | 4 | 50553134d558 | `cases/cpb-dyn-c-n02-flat--abs-srss` | [ ] |
| 43 | 19 | B, table typed | cpb-dyn-c-n02-flat--abs-srss-table-typed | 4 | 588faa9a541e | `cases/cpb-dyn-c-n02-flat--abs-srss-table-typed` | [ ] |
| 44 | 20 | A, table added | cpb-dyn-c-n02-flat--srss-abs | 4 | d98c07bccb81 | `cases/cpb-dyn-c-n02-flat--srss-abs` | [ ] |
| 45 | 20 | B, table typed | cpb-dyn-c-n02-flat--srss-abs-table-typed | 4 | 89771c2bd876 | `cases/cpb-dyn-c-n02-flat--srss-abs-table-typed` | [ ] |
| 46 | 21 | A, table added | cpb-dyn-c-n02-flat--closely-spaced-abs | 4 | 6ee90cf854f2 | `cases/cpb-dyn-c-n02-flat--closely-spaced-abs` | [ ] |
| 47 | 21 | B, table typed | cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed | 4 | 0b4d3587afb7 | `cases/cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed` | [ ] |
| 48 | 22 | A, table added | cpb-dyn-c-n02-flat--closely-spaced-srss | 4 | 912c9d028e3e | `cases/cpb-dyn-c-n02-flat--closely-spaced-srss` | [ ] |
| 49 | 22 | B, table typed | cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed | 4 | a67bfef56f56 | `cases/cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed` | [ ] |
| 50 | 23 | A, table added | cpb-dyn-c-n02-flat--nrl-abs | 4 | bce7b8eb2677 | `cases/cpb-dyn-c-n02-flat--nrl-abs` | [ ] |
| 51 | 23 | B, table typed | cpb-dyn-c-n02-flat--nrl-abs-table-typed | 4 | 09e4a6f0b3c0 | `cases/cpb-dyn-c-n02-flat--nrl-abs-table-typed` | [ ] |
| 52 | 24 | A, table added | cpb-dyn-c-n02-flat--nrl-srss | 4 | f59c2b0602b3 | `cases/cpb-dyn-c-n02-flat--nrl-srss` | [ ] |
| 53 | 24 | B, table typed | cpb-dyn-c-n02-flat--nrl-srss-table-typed | 4 | 06be0a1388d9 | `cases/cpb-dyn-c-n02-flat--nrl-srss-table-typed` | [ ] |
| 54 | 25 | A, table added | cpb-dyn-c-n16-flat--abs-abs | 18 | e55759f1456f | `cases/cpb-dyn-c-n16-flat--abs-abs` | [ ] |
| 55 | 25 | B, table typed | cpb-dyn-c-n16-flat--abs-abs-table-typed | 18 | aefbc0ccb0fd | `cases/cpb-dyn-c-n16-flat--abs-abs-table-typed` | [ ] |
| 56 | 26 | A, table added | cpb-dyn-c-n16-flat--abs-srss | 18 | 8215f8b1d7ba | `cases/cpb-dyn-c-n16-flat--abs-srss` | [ ] |
| 57 | 26 | B, table typed | cpb-dyn-c-n16-flat--abs-srss-table-typed | 18 | a19cd8258939 | `cases/cpb-dyn-c-n16-flat--abs-srss-table-typed` | [ ] |
| 58 | 27 | A, table added | cpb-dyn-c-n16-flat--srss-abs | 18 | e5d3990152b1 | `cases/cpb-dyn-c-n16-flat--srss-abs` | [ ] |
| 59 | 27 | B, table typed | cpb-dyn-c-n16-flat--srss-abs-table-typed | 18 | fbe06dec10f7 | `cases/cpb-dyn-c-n16-flat--srss-abs-table-typed` | [ ] |
| 60 | 28 | A, table added | cpb-dyn-c-n16-flat--closely-spaced-abs | 18 | 64e344c00784 | `cases/cpb-dyn-c-n16-flat--closely-spaced-abs` | [ ] |
| 61 | 28 | B, table typed | cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed | 18 | 7273e666a8ca | `cases/cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed` | [ ] |
| 62 | 29 | A, table added | cpb-dyn-c-n16-flat--closely-spaced-srss | 18 | 1c4ef2e3ffe2 | `cases/cpb-dyn-c-n16-flat--closely-spaced-srss` | [ ] |
| 63 | 29 | B, table typed | cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed | 18 | 4c1f658a1c47 | `cases/cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed` | [ ] |
| 64 | 30 | A, table added | cpb-dyn-c-n16-flat--nrl-abs | 18 | 610112ffe661 | `cases/cpb-dyn-c-n16-flat--nrl-abs` | [ ] |
| 65 | 30 | B, table typed | cpb-dyn-c-n16-flat--nrl-abs-table-typed | 18 | ba51052aeb05 | `cases/cpb-dyn-c-n16-flat--nrl-abs-table-typed` | [ ] |
| 66 | 31 | A, table added | cpb-dyn-c-n16-flat--nrl-srss | 18 | e42c47d0971b | `cases/cpb-dyn-c-n16-flat--nrl-srss` | [ ] |
| 67 | 31 | B, table typed | cpb-dyn-c-n16-flat--nrl-srss-table-typed | 18 | b246574bcc60 | `cases/cpb-dyn-c-n16-flat--nrl-srss-table-typed` | [ ] |
| 68 | 32 | A, table added | cpb-dyn-d-n08-flat--abs-srss | 10 | 0dc42f1a398d | `cases/cpb-dyn-d-n08-flat--abs-srss` | [ ] |
| 69 | 32 | B, table typed | cpb-dyn-d-n08-flat--abs-srss-table-typed | 10 | a6b5587ec832 | `cases/cpb-dyn-d-n08-flat--abs-srss-table-typed` | [ ] |
| 70 | 33 | A, table added | cpb-dyn-d-n08-flat--closely-spaced-srss | 10 | 4f01742a9d95 | `cases/cpb-dyn-d-n08-flat--closely-spaced-srss` | [ ] |
| 71 | 33 | B, table typed | cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed | 10 | a3db36ac7e74 | `cases/cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed` | [ ] |
| 72 | 34 | A, table added | cpb-dyn-d-n08-flat--nrl-srss | 10 | f3fdbc728c7c | `cases/cpb-dyn-d-n08-flat--nrl-srss` | [ ] |
| 73 | 34 | B, table typed | cpb-dyn-d-n08-flat--nrl-srss-table-typed | 10 | badd9c1fded1 | `cases/cpb-dyn-d-n08-flat--nrl-srss-table-typed` | [ ] |

## Part 2: Pressurised series

The declared screened runs of the cases that carry the exact pressure contract, each model's pressure-default run next to its pressure-elongation-on run, then the declared capture-only runs (pressure-bourdon-expansion-capture), which are sealed captures.

Evidence of the model run: whatever the program writes beside the model file its batch line names, brought back in that folder with the print; which files those are is the dry run's answer A (INSTRUCTIONS.md section 2).

Batch lines: group 2 of `BATCH_COMMANDS.txt` after the two set-up lines, 66 line(s), pasted one line at a time in this order.

| # | item | kind | layout rows | MBF SHA-256 (first 12) | folder | done |
|---|---|---|---|---|---|---|
| 74 | cpb-p1-01--pressure-default | benchmark case | 3 | 8bd632624f51 | `cases/cpb-p1-01--pressure-default` | [ ] |
| 75 | cpb-p1-01--pressure-elongation-on | benchmark case | 3 | 0060111ea908 | `cases/cpb-p1-01--pressure-elongation-on` | [ ] |
| 76 | cpb-p1-02--pressure-default | benchmark case | 3 | 89172c315a56 | `cases/cpb-p1-02--pressure-default` | [ ] |
| 77 | cpb-p1-02--pressure-elongation-on | benchmark case | 3 | bcf3a7678b45 | `cases/cpb-p1-02--pressure-elongation-on` | [ ] |
| 78 | cpb-p1-03--pressure-default | benchmark case | 3 | 3c0ef6733f19 | `cases/cpb-p1-03--pressure-default` | [ ] |
| 79 | cpb-p1-03--pressure-elongation-on | benchmark case | 3 | 1b22d0361518 | `cases/cpb-p1-03--pressure-elongation-on` | [ ] |
| 80 | cpb-p1-04--pressure-default | benchmark case | 3 | 81e6bd79a949 | `cases/cpb-p1-04--pressure-default` | [ ] |
| 81 | cpb-p1-04--pressure-elongation-on | benchmark case | 3 | 82e7f30c414f | `cases/cpb-p1-04--pressure-elongation-on` | [ ] |
| 82 | cpb-p1-05--pressure-default | benchmark case | 3 | 3877f68151cd | `cases/cpb-p1-05--pressure-default` | [ ] |
| 83 | cpb-p1-05--pressure-elongation-on | benchmark case | 3 | 04e907b618dd | `cases/cpb-p1-05--pressure-elongation-on` | [ ] |
| 84 | cpb-p1-06--pressure-default | benchmark case | 3 | 453d90943e51 | `cases/cpb-p1-06--pressure-default` | [ ] |
| 85 | cpb-p1-06--pressure-elongation-on | benchmark case | 3 | de768ba109dc | `cases/cpb-p1-06--pressure-elongation-on` | [ ] |
| 86 | cpb-p2-01--pressure-default | benchmark case | 3 | b0eecb1e0a2c | `cases/cpb-p2-01--pressure-default` | [ ] |
| 87 | cpb-p2-01--pressure-elongation-on | benchmark case | 3 | 583879e0c346 | `cases/cpb-p2-01--pressure-elongation-on` | [ ] |
| 88 | cpb-p2-02--pressure-default | benchmark case | 3 | deda33b583de | `cases/cpb-p2-02--pressure-default` | [ ] |
| 89 | cpb-p2-02--pressure-elongation-on | benchmark case | 3 | af2141bfcbe3 | `cases/cpb-p2-02--pressure-elongation-on` | [ ] |
| 90 | cpb-p2-03--pressure-default | benchmark case | 3 | c962bc727035 | `cases/cpb-p2-03--pressure-default` | [ ] |
| 91 | cpb-p2-03--pressure-elongation-on | benchmark case | 3 | 214d98b0e6b2 | `cases/cpb-p2-03--pressure-elongation-on` | [ ] |
| 92 | cpb-p2-04--pressure-default | benchmark case | 3 | b68fa43b3651 | `cases/cpb-p2-04--pressure-default` | [ ] |
| 93 | cpb-p2-04--pressure-elongation-on | benchmark case | 3 | cb0a6f56325c | `cases/cpb-p2-04--pressure-elongation-on` | [ ] |
| 94 | cpb-p3-01--pressure-default | benchmark case | 3 | 964d60f3f2d3 | `cases/cpb-p3-01--pressure-default` | [ ] |
| 95 | cpb-p3-01--pressure-elongation-on | benchmark case | 3 | eb2454d77c36 | `cases/cpb-p3-01--pressure-elongation-on` | [ ] |
| 96 | cpb-p3-02--pressure-default | benchmark case | 3 | 54d76b6fb6a6 | `cases/cpb-p3-02--pressure-default` | [ ] |
| 97 | cpb-p3-02--pressure-elongation-on | benchmark case | 3 | ccc61e0b87ae | `cases/cpb-p3-02--pressure-elongation-on` | [ ] |
| 98 | cpb-p3-03--pressure-default | benchmark case | 3 | 989f5e200a0c | `cases/cpb-p3-03--pressure-default` | [ ] |
| 99 | cpb-p3-03--pressure-elongation-on | benchmark case | 3 | b96d2ee11cd9 | `cases/cpb-p3-03--pressure-elongation-on` | [ ] |
| 100 | cpb-p3-04--pressure-default | benchmark case | 3 | ed708470595f | `cases/cpb-p3-04--pressure-default` | [ ] |
| 101 | cpb-p3-04--pressure-elongation-on | benchmark case | 3 | 83d156ba4157 | `cases/cpb-p3-04--pressure-elongation-on` | [ ] |
| 102 | cpb-p4-01--pressure-default | benchmark case | 4 | 6d8c7ba785e0 | `cases/cpb-p4-01--pressure-default` | [ ] |
| 103 | cpb-p4-01--pressure-elongation-on | benchmark case | 4 | 8ec1f528811b | `cases/cpb-p4-01--pressure-elongation-on` | [ ] |
| 104 | cpb-p4-02--pressure-default | benchmark case | 4 | af77c4759efc | `cases/cpb-p4-02--pressure-default` | [ ] |
| 105 | cpb-p4-02--pressure-elongation-on | benchmark case | 4 | 34db8980a74a | `cases/cpb-p4-02--pressure-elongation-on` | [ ] |
| 106 | cpb-p4-03--pressure-default | benchmark case | 4 | fb9caf3b5562 | `cases/cpb-p4-03--pressure-default` | [ ] |
| 107 | cpb-p4-03--pressure-elongation-on | benchmark case | 4 | 757e46adb750 | `cases/cpb-p4-03--pressure-elongation-on` | [ ] |
| 108 | cpb-p4-04--pressure-default | benchmark case | 4 | 4df788e8e36e | `cases/cpb-p4-04--pressure-default` | [ ] |
| 109 | cpb-p4-04--pressure-elongation-on | benchmark case | 4 | fbc19693d996 | `cases/cpb-p4-04--pressure-elongation-on` | [ ] |
| 110 | cpb-p5-01--pressure-default | benchmark case | 5 | 3c4cdba57f9c | `cases/cpb-p5-01--pressure-default` | [ ] |
| 111 | cpb-p5-01--pressure-elongation-on | benchmark case | 5 | 474ec3482926 | `cases/cpb-p5-01--pressure-elongation-on` | [ ] |
| 112 | cpb-p5-02--pressure-default | benchmark case | 5 | eaf212a32caa | `cases/cpb-p5-02--pressure-default` | [ ] |
| 113 | cpb-p5-02--pressure-elongation-on | benchmark case | 5 | 0993ca66c468 | `cases/cpb-p5-02--pressure-elongation-on` | [ ] |
| 114 | cpb-p6-01--pressure-default | benchmark case | 6 | 8f1dd1f03994 | `cases/cpb-p6-01--pressure-default` | [ ] |
| 115 | cpb-p6-01--pressure-elongation-on | benchmark case | 6 | 1f74e1124bd7 | `cases/cpb-p6-01--pressure-elongation-on` | [ ] |
| 116 | cpb-p7-01--pressure-default | benchmark case | 4 | d50585b9b501 | `cases/cpb-p7-01--pressure-default` | [ ] |
| 117 | cpb-p7-01--pressure-elongation-on | benchmark case | 4 | e0b0199ab109 | `cases/cpb-p7-01--pressure-elongation-on` | [ ] |
| 118 | cpb-p1-01--pressure-bourdon-expansion-capture | sealed capture | 3 | 022a6389082f | `cases/cpb-p1-01--pressure-bourdon-expansion-capture` | [ ] |
| 119 | cpb-p1-02--pressure-bourdon-expansion-capture | sealed capture | 3 | 144f79639c6b | `cases/cpb-p1-02--pressure-bourdon-expansion-capture` | [ ] |
| 120 | cpb-p1-03--pressure-bourdon-expansion-capture | sealed capture | 3 | 061c614696f5 | `cases/cpb-p1-03--pressure-bourdon-expansion-capture` | [ ] |
| 121 | cpb-p1-04--pressure-bourdon-expansion-capture | sealed capture | 3 | ac3625f47a39 | `cases/cpb-p1-04--pressure-bourdon-expansion-capture` | [ ] |
| 122 | cpb-p1-05--pressure-bourdon-expansion-capture | sealed capture | 3 | 4f3aecb3503f | `cases/cpb-p1-05--pressure-bourdon-expansion-capture` | [ ] |
| 123 | cpb-p1-06--pressure-bourdon-expansion-capture | sealed capture | 3 | f11e1e470dff | `cases/cpb-p1-06--pressure-bourdon-expansion-capture` | [ ] |
| 124 | cpb-p2-01--pressure-bourdon-expansion-capture | sealed capture | 3 | 334b14c9005f | `cases/cpb-p2-01--pressure-bourdon-expansion-capture` | [ ] |
| 125 | cpb-p2-02--pressure-bourdon-expansion-capture | sealed capture | 3 | 0ea90db3bf32 | `cases/cpb-p2-02--pressure-bourdon-expansion-capture` | [ ] |
| 126 | cpb-p2-03--pressure-bourdon-expansion-capture | sealed capture | 3 | 7f111d691de3 | `cases/cpb-p2-03--pressure-bourdon-expansion-capture` | [ ] |
| 127 | cpb-p2-04--pressure-bourdon-expansion-capture | sealed capture | 3 | 45e0bfbc1010 | `cases/cpb-p2-04--pressure-bourdon-expansion-capture` | [ ] |
| 128 | cpb-p3-01--pressure-bourdon-expansion-capture | sealed capture | 3 | 5829ea3272f3 | `cases/cpb-p3-01--pressure-bourdon-expansion-capture` | [ ] |
| 129 | cpb-p3-02--pressure-bourdon-expansion-capture | sealed capture | 3 | 8e2021ed23b2 | `cases/cpb-p3-02--pressure-bourdon-expansion-capture` | [ ] |
| 130 | cpb-p3-03--pressure-bourdon-expansion-capture | sealed capture | 3 | 2de682787dae | `cases/cpb-p3-03--pressure-bourdon-expansion-capture` | [ ] |
| 131 | cpb-p3-04--pressure-bourdon-expansion-capture | sealed capture | 3 | af85adadbf40 | `cases/cpb-p3-04--pressure-bourdon-expansion-capture` | [ ] |
| 132 | cpb-p4-01--pressure-bourdon-expansion-capture | sealed capture | 4 | 702e2cd7f0af | `cases/cpb-p4-01--pressure-bourdon-expansion-capture` | [ ] |
| 133 | cpb-p4-02--pressure-bourdon-expansion-capture | sealed capture | 4 | bf8291209ecd | `cases/cpb-p4-02--pressure-bourdon-expansion-capture` | [ ] |
| 134 | cpb-p4-03--pressure-bourdon-expansion-capture | sealed capture | 4 | 5214d2c3a473 | `cases/cpb-p4-03--pressure-bourdon-expansion-capture` | [ ] |
| 135 | cpb-p4-04--pressure-bourdon-expansion-capture | sealed capture | 4 | 8b794b23f8ea | `cases/cpb-p4-04--pressure-bourdon-expansion-capture` | [ ] |
| 136 | cpb-p5-01--pressure-bourdon-expansion-capture | sealed capture | 5 | 8897fbe02643 | `cases/cpb-p5-01--pressure-bourdon-expansion-capture` | [ ] |
| 137 | cpb-p5-02--pressure-bourdon-expansion-capture | sealed capture | 5 | 39f60c556b01 | `cases/cpb-p5-02--pressure-bourdon-expansion-capture` | [ ] |
| 138 | cpb-p6-01--pressure-bourdon-expansion-capture | sealed capture | 6 | 26e2cc7f1dd2 | `cases/cpb-p6-01--pressure-bourdon-expansion-capture` | [ ] |
| 139 | cpb-p7-01--pressure-bourdon-expansion-capture | sealed capture | 4 | d8028407d41b | `cases/cpb-p7-01--pressure-bourdon-expansion-capture` | [ ] |

## Part 3: Series S1, S2 and S3

The three linear static one-parameter series of tranche 3: transverse shear in straight pipe (S1), shear and flexibility in a bend (S2) and the finite stiffness of a rigid support (S3), each series' members together.

Evidence of the model run: whatever the program writes beside the model file its batch line names, brought back in that folder with the print; which files those are is the dry run's answer A (INSTRUCTIONS.md section 2).

Batch lines: group 3 of `BATCH_COMMANDS.txt` after the two set-up lines, 12 line(s), pasted one line at a time in this order.

| # | item | kind | layout rows | MBF SHA-256 (first 12) | folder | done |
|---|---|---|---|---|---|---|
| 140 | cpb-s1-01 | benchmark case | 4 | 9643c4b24f70 | `cases/cpb-s1-01` | [ ] |
| 141 | cpb-s1-02 | benchmark case | 4 | 32e276a73330 | `cases/cpb-s1-02` | [ ] |
| 142 | cpb-s1-03 | benchmark case | 4 | 1d5684109366 | `cases/cpb-s1-03` | [ ] |
| 143 | cpb-s1-04 | benchmark case | 4 | 2644b636f8ed | `cases/cpb-s1-04` | [ ] |
| 144 | cpb-s2-01 | benchmark case | 4 | 3234e246bf06 | `cases/cpb-s2-01` | [ ] |
| 145 | cpb-s2-02 | benchmark case | 4 | fd0cfd9db524 | `cases/cpb-s2-02` | [ ] |
| 146 | cpb-s2-03 | benchmark case | 4 | 5a394be2b1f4 | `cases/cpb-s2-03` | [ ] |
| 147 | cpb-s3-01 | benchmark case | 4 | 372e6d54782b | `cases/cpb-s3-01` | [ ] |
| 148 | cpb-s3-02 | benchmark case | 4 | 1539900a0160 | `cases/cpb-s3-02` | [ ] |
| 149 | cpb-s3-03 | benchmark case | 4 | ac4de7fb83ea | `cases/cpb-s3-03` | [ ] |
| 150 | cpb-s3-04 | benchmark case | 4 | 811b38a375fa | `cases/cpb-s3-04` | [ ] |
| 151 | cpb-s3-05 | benchmark case | 4 | a0c5e9073718 | `cases/cpb-s3-05` | [ ] |

## Part 4: Hand-entry cases

The five cases whose one support member is entered by hand on a written base, in the benchmark manifest's rank order, in the program's window only.

Evidence of the model run: the model saved as `<run id>.mod` from inside the program before the analysis, brought back in the run's folder with its print (INSTRUCTIONS.md section 2).

Window only: no case of this part has a line in `BATCH_COMMANDS.txt`.

| # | item | kind | layout rows | MBF SHA-256 (first 12) | folder | done |
|---|---|---|---|---|---|---|
| 152 | cpb-n-01--stop-axial-closed | held capture, window only | 4 | 45f7b1f0ee42 | `cases/cpb-n-01--stop-axial-closed` | [ ] |
| 153 | cpb-n-02--rest-lift-off | held capture, window only | 4 | 12ed404f8639 | `cases/cpb-n-02--rest-lift-off` | [ ] |
| 154 | cpb-n-03--rest-closed | held capture, window only | 4 | 437f21ebf4b8 | `cases/cpb-n-03--rest-closed` | [ ] |
| 155 | cpb-n-04--stop-axial-open | held capture, window only | 4 | bb3d3f89210e | `cases/cpb-n-04--stop-axial-open` | [ ] |
| 156 | cpb-h-01--constant-support | held capture, window only | 4 | 9faa173dc693 | `cases/cpb-h-01--constant-support` | [ ] |

## Part 5: Tranche 1 remainder

The first tranche's other models, by family, the members of one series next to each other and smaller models first.

Evidence of the model run: whatever the program writes beside the model file its batch line names, brought back in that folder with the print; which files those are is the dry run's answer A (INSTRUCTIONS.md section 2).

Batch lines: group 4 of `BATCH_COMMANDS.txt` after the two set-up lines, 57 line(s), pasted one line at a time in this order.

| # | item | kind | layout rows | MBF SHA-256 (first 12) | folder | done |
|---|---|---|---|---|---|---|
| 157 | cpb-g1-01 | benchmark case | 4 | f73904d6565b | `cases/cpb-g1-01` | [ ] |
| 158 | cpb-g1-02 | benchmark case | 4 | cfc581c73517 | `cases/cpb-g1-02` | [ ] |
| 159 | cpb-g1-03 | benchmark case | 4 | c188bf321976 | `cases/cpb-g1-03` | [ ] |
| 160 | cpb-g1-04 | benchmark case | 4 | 8e61efb6397a | `cases/cpb-g1-04` | [ ] |
| 161 | cpb-g1-06 | benchmark case | 4 | e9e41929f084 | `cases/cpb-g1-06` | [ ] |
| 162 | cpb-g1-07 | benchmark case | 4 | 90e12e9600f3 | `cases/cpb-g1-07` | [ ] |
| 163 | cpb-g1-10 | benchmark case | 4 | 6381eb2903db | `cases/cpb-g1-10` | [ ] |
| 164 | cpb-g1-08 | benchmark case | 6 | 70efb097970c | `cases/cpb-g1-08` | [ ] |
| 165 | cpb-g1-09 | benchmark case | 8 | f66a29feacae | `cases/cpb-g1-09` | [ ] |
| 166 | cpb-g2-01 | benchmark case | 5 | a631adabe721 | `cases/cpb-g2-01` | [ ] |
| 167 | cpb-g2-02 | benchmark case | 5 | c3a1336a055d | `cases/cpb-g2-02` | [ ] |
| 168 | cpb-g2-03 | benchmark case | 5 | de43594eb899 | `cases/cpb-g2-03` | [ ] |
| 169 | cpb-g2-04 | benchmark case | 5 | 3a0d1da7a40c | `cases/cpb-g2-04` | [ ] |
| 170 | cpb-g2-05 | benchmark case | 5 | b6ae8bd5900d | `cases/cpb-g2-05` | [ ] |
| 171 | cpb-g2-06 | benchmark case | 5 | c9d89a0ff839 | `cases/cpb-g2-06` | [ ] |
| 172 | cpb-g2-07 | benchmark case | 5 | dfa83a259582 | `cases/cpb-g2-07` | [ ] |
| 173 | cpb-g2-08 | benchmark case | 5 | 7a7e67eb458b | `cases/cpb-g2-08` | [ ] |
| 174 | cpb-g2-09 | benchmark case | 5 | c8b5c4d801a9 | `cases/cpb-g2-09` | [ ] |
| 175 | cpb-g2-10 | benchmark case | 5 | 1950ef61637c | `cases/cpb-g2-10` | [ ] |
| 176 | cpb-g2-11 | benchmark case | 5 | 01df291913cc | `cases/cpb-g2-11` | [ ] |
| 177 | cpb-g2-12 | benchmark case | 5 | 062383dc7894 | `cases/cpb-g2-12` | [ ] |
| 178 | cpb-g2-13 | benchmark case | 5 | 9f2a5e547c61 | `cases/cpb-g2-13` | [ ] |
| 179 | cpb-g2-14 | benchmark case | 5 | e0c6b73eba17 | `cases/cpb-g2-14` | [ ] |
| 180 | cpb-g2-15 | benchmark case | 5 | 58cc9bda4262 | `cases/cpb-g2-15` | [ ] |
| 181 | cpb-g2-16 | benchmark case | 5 | 1338c17dd81a | `cases/cpb-g2-16` | [ ] |
| 182 | cpb-g3-01 | benchmark case | 6 | 4cc92d192206 | `cases/cpb-g3-01` | [ ] |
| 183 | cpb-g3-02 | benchmark case | 6 | 3bf7eb423959 | `cases/cpb-g3-02` | [ ] |
| 184 | cpb-g3-03 | benchmark case | 6 | b01799779eec | `cases/cpb-g3-03` | [ ] |
| 185 | cpb-g3-04 | benchmark case | 6 | fa9a746c4740 | `cases/cpb-g3-04` | [ ] |
| 186 | cpb-g3-05 | benchmark case | 6 | e7079ab044a2 | `cases/cpb-g3-05` | [ ] |
| 187 | cpb-g3-06 | benchmark case | 6 | 009d5755ebc7 | `cases/cpb-g3-06` | [ ] |
| 188 | cpb-g3-07 | benchmark case | 8 | 962621743746 | `cases/cpb-g3-07` | [ ] |
| 189 | cpb-g3-08 | benchmark case | 8 | b003c6c901e3 | `cases/cpb-g3-08` | [ ] |
| 190 | cpb-g3-09 | benchmark case | 8 | 7f157292125d | `cases/cpb-g3-09` | [ ] |
| 191 | cpb-g3-10 | benchmark case | 8 | 1efd72f9f1f2 | `cases/cpb-g3-10` | [ ] |
| 192 | cpb-g3-11 | benchmark case | 8 | 29ff233c3563 | `cases/cpb-g3-11` | [ ] |
| 193 | cpb-g3-12 | benchmark case | 8 | aa7b9f8eb85b | `cases/cpb-g3-12` | [ ] |
| 194 | cpb-g4-01 | benchmark case | 6 | e300c9d6b2c8 | `cases/cpb-g4-01` | [ ] |
| 195 | cpb-g4-02 | benchmark case | 6 | cbf301636f10 | `cases/cpb-g4-02` | [ ] |
| 196 | cpb-g4-03 | benchmark case | 6 | 44e7988f8750 | `cases/cpb-g4-03` | [ ] |
| 197 | cpb-g4-04 | benchmark case | 6 | 6e2bd7305bba | `cases/cpb-g4-04` | [ ] |
| 198 | cpb-g4-05 | benchmark case | 6 | 22d58b2acccd | `cases/cpb-g4-05` | [ ] |
| 199 | cpb-g4-06 | benchmark case | 6 | 578538074174 | `cases/cpb-g4-06` | [ ] |
| 200 | cpb-g5-01 | benchmark case | 7 | c171214833f8 | `cases/cpb-g5-01` | [ ] |
| 201 | cpb-g5-02 | benchmark case | 7 | 755896c5b833 | `cases/cpb-g5-02` | [ ] |
| 202 | cpb-g5-03 | benchmark case | 7 | 28286bdfcb06 | `cases/cpb-g5-03` | [ ] |
| 203 | cpb-g5-04 | benchmark case | 7 | 68fff300e7a5 | `cases/cpb-g5-04` | [ ] |
| 204 | cpb-g5-05 | benchmark case | 7 | cb1e4c05ce1f | `cases/cpb-g5-05` | [ ] |
| 205 | cpb-g5-06 | benchmark case | 7 | 0fbdcb1a7cfd | `cases/cpb-g5-06` | [ ] |
| 206 | cpb-g6-01 | benchmark case | 7 | 940253ebf794 | `cases/cpb-g6-01` | [ ] |
| 207 | cpb-g6-03 | benchmark case | 7 | 8c1250d9af4e | `cases/cpb-g6-03` | [ ] |
| 208 | cpb-g7-01 | benchmark case | 8 | e10e69983bca | `cases/cpb-g7-01` | [ ] |
| 209 | cpb-g7-02 | benchmark case | 8 | 47a1f0b7183b | `cases/cpb-g7-02` | [ ] |
| 210 | cpb-g7-03 | benchmark case | 8 | af1bb291e96a | `cases/cpb-g7-03` | [ ] |
| 211 | cpb-g8-02 | benchmark case | 4 | 18a908fec0a1 | `cases/cpb-g8-02` | [ ] |
| 212 | cpb-g8-03 | benchmark case | 4 | e2b879e2e710 | `cases/cpb-g8-03` | [ ] |
| 213 | cpb-g8-04 | benchmark case | 4 | 29ee08338d4e | `cases/cpb-g8-04` | [ ] |
