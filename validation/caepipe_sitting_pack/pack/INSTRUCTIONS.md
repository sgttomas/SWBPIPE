Development comparison only. In any SWBPIPE result, 'checks passed', 'established' and certified bounds rest on a premise the product does not verify, which is false on a constructed model (DEL-04-01; exit criterion 18, not met). They are not established engineering results. The mechanics values are compared with CAEPIPE on that understanding.

Nothing in this pack is a SWBPIPE result. Where a case is held until the product publishes it as checks passed, that phrase names the product's own gate and carries this qualification.

# SWBPIPE sitting: instructions

This pack is not a SWBPIPE result and holds none. It carries no driver, no script that runs at the sitting and no part of SWBPIPE's harness: you run every case by hand, in your own licensed CAEPIPE.

The pack holds the cases SWBPIPE asks you to run, in the order of `RUN_SHEET.md`. Every model file in it is the output of SWBPIPE's benchmark generator (`tools/caepipe_correlation/benchmark_models.py` with its tranche modules) through SWBPIPE's model-file writer. The first and second tranches' members are drawn from seed 20261028; the third tranche's members are designed, each stated in the generator's own text, and draw nothing from the seed. For a run the benchmark manifest declares, the file is that run's: the written file, with its pressure options line substituted or whole options records added where the manifest declares that. The same inputs give the same bytes. The pack contains only its specified permitted kinds of file and reviewed contents: model files (`.mbf`), records (`.json`), these instructions and the run sheet (`.md`), and plain text (`.txt`). Its `pack_index.json` names every file of this folder, and of the `records` folder that stays on the Mac, with its SHA-256 and size.

**The results are private.** Every file that comes back in this pack is private. Keep the returned pack in your exchange folder or another private place outside every repository. Never commit, share or upload it.

**On the Mac.** No SWBPIPE command for the returned folder is part of this pack. On the Mac, the storage procedure copies the returned folder under one new name, records each file's path, size and hash, and verifies the copied contents against the source hashes. It reads file contents as bytes without interpreting prints or run lines, comparing models with the pack, or classifying files by kind. This procedure checks no print sections. Your dry-run comparison checks the example print you inspect; it does not check subsequent prints.

**What a result waits on.** A result from this sitting is screened only after (a) the harness's stored-results change (pull request 262) has merged, (b) a rule for a returned file's origin is in place, and (c) the comparison record itself carries the qualification of SWBPIPE's results as a required member (pull request 263); no screen is run before all three. A sealed capture is run like any other case and brought back; nothing in this pack unseals it.

## 1. Before you start

> **Never leave the pressure options line blank.** When you import or edit a model by hand, leave the line under the `OPPRES` heading (the pressure options) exactly as it is in the pack's model file, and never clear it. The vendor's public import page (CAEPIPE-IMPORT-MBF, https://docs.sstcae.com/users_manual/import_mbf.htm, "Format of .MBF") lists among that line's fields "[Bourdon effect (0 = Off, 1 = ON)], [Pressure Correction for Bends (0 = Off, 1 = On)]" and says: "Leaving a BLANK line following the Section heading “OPPRES” will set the values as 0, 1, 1, 1.00, 0."

1. Copy the pack folder (`pack`, the folder that holds these instructions) and nothing else to a fresh working folder on the Windows machine, at a short path of letters, digits and hyphens only, for example `C:\swbpipe-sitting`. Choose a working folder that is not the exchange folder and is not inside it. Run nothing from the exchange folder. The `records` folder beside the pack folder stays on the Mac; never copy it.
2. Do not rename any folder or file the pack holds. Each case and sample folder holds one small provenance file, `<stem>.provenance.json`, beside its model file. Do not open it in the program; you need not bring it back.
3. Work down `RUN_SHEET.md` part by part, in order. Each case has its own folder; do everything for that case in it.
4. If the sitting must be shortened, strike parts from the end: part 5 first, then part 4, part 3 and part 2. Part 1 is never struck to make room for a later part. If time runs out within it, stop after a complete strike number: the lowest numbers are run first, and strike numbers 32 to 34 are the first to go.

## 2. What binds a result to the model that was run

A matching model hash shows which bytes were kept, not which model CAEPIPE actually ran, and evidence that is not made at the sitting cannot be made afterwards. What each kind of run makes, and what to bring back:

- **A window run** (parts 1 and 4, and a batch case run the window way): save the model under the run's name, `<run id>.mod`, from inside the program before the analysis, in the run's folder, and bring it back there with its print. That saved model is the evidence.
- **A batch run** (parts 2, 3 and 5, and the dry run's cases): the command line names the model file; bring back whatever the program writes beside it, in the same folder, with the print. Which files those are is the dry run's answer A. If answer A shows that a batch run leaves a model file of the program's own beside the print, that file is the evidence. If it shows none, a batch run's only evidence is that its print came back in the folder of the model file its command line named, and that is weaker evidence.
- **A run made by the one-session alternative:** the model saved under that run's name after the setting was changed, with the note on its run line, is the evidence.

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

An export, where one is made, is bound to that run's saved model and that run's instruction: the program writes it from that run's saved model (`<run id>.mod`) as the program holds it after that run's instruction, including what was entered or set by hand; it is not the pack's model file, and it is not evidence of which model was run.

**Exports.** The runs that offer an export, from the benchmark manifest: the five hand-entry runs and every dynamics run at strike numbers 1 to 10 (both forms of each pair and the single typed form at strike 6); no other run. Of an export, the benchmark manifest says: "an export is a model batch file the program writes, at the owner's choice, from a run's saved model (<run id>.mod) as the program holds it after that run's instruction, including what was entered or set by hand; it is not the pack's model file. Each export is private, like every returned file. The exports are material for later interoperability work: they put on file how the program itself writes records SWBPIPE's writer does not write today. They do not demonstrate exit criterion 6's round trip in the desktop, and they do not make any family the product refuses become supported. Nothing of SWBPIPE's reads an export in this change." Each export's steps are optional, and you may drop them; if you do, write "export not made" on the run line. A run whose export is dropped is still run.

## 3. Two ways to run a case

Each case is offered one or both of two ways (`manual_batch`, `manual_print`), named for it in section 5. A window-only case is offered the program's window alone and has no line in `BATCH_COMMANDS.txt`. Where a case is offered both ways, run it by its batch line; use the window way for that case only if its batch line left no results, and note that on its run line.

### 3.1 By hand, batch

The batch page (CAEPIPE-BATCH, https://docs.sstcae.com/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm) says: "Batch mode requires two arguments, separated by a semicolon (;). The first argument is the MBF file name, and the second argument is batchmode=yes." Of its example it says: "The results will be output in CSV file format after analysis with name “StrModel R0.csv” and will be stored in the folder where the MBF file is stored". Which files appear beside the model file is observed at the dry run (answer A).

`BATCH_COMMANDS.txt` holds two set-up lines, then one line for every case this way can run, in the order of `RUN_SHEET.md`, each part's lines together and a blank line between parts. Open a Command Prompt. Type the first set-up line, with the full path of your CAEPIPE program in place of the text in angle brackets:

```
set "CAEPIPE=<the full path of your CAEPIPE program>"
```

Then type the second set-up line, with the full path of the pack folder in place of the text in angle brackets:

```
cd /d "<the full path of the pack folder>"
```

Then paste the case lines one line at a time, in the file's order. Every case line has this form and nothing else:

```
start "" /wait "%CAEPIPE%" "%CD%\<folder>\<stem>.mbf";batchmode=yes
```

Microsoft's page for `start` (MS-START, https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/start) gives the words for its two parts used here: for `"title"` (the empty `""`), "Specifies the title to display in the Command Prompt window title bar."; and for `/wait`, "Starts an application and waits for it to end." Paste the next line only when the program has ended and the case's results are in its folder.

If a window of the program asks something or shows a message, answering or closing it is yours to do, by hand; nothing in the pack answers it. If a case ends with no results in its folder, note it on its run line, then run that case the window way (3.2) before pasting the next line. The import may write over the model file the batch line left in that folder (the export page's words in 3.2, step 1); note on the run line that the case was imported after its batch line.

### 3.2 By hand, in the program's window

1. Import `<stem>.mbf` with the program's import command (the import page, CAEPIPE-IMPORT-MBF, https://docs.sstcae.com/users_manual/import_mbf.htm: "To import a model, select the menu command File > Import from the Main window."). The export page (CAEPIPE-EXPORT-MBF, https://docs.sstcae.com/users_manual/export_mbf.htm) says: "But note that the existing .mod file will be overwritten. So, you must rename the edited text file before importing it back into CAEPIPE." The page does not say in which folder. Each run has its own folder for that reason. Then save the model as `<stem>.mod` in this folder by the command you noted at the dry run (check (iii)). Do not save over the imported file `<stem>.mbf`.
2. Before analysing, compare the model's settings as the window shows them with the case's settings in section 5: piping code, B31J, reference temperature, units, vertical axis, gravity and the pressure options. Do not edit the model file. If the window shows a different value, write the value it shows on the run line.
3. Analyse the model by File > Analyze (F12). The Analyze page (CAEPIPE-ANALYZE, https://docs.sstcae.com/users_manual/analyze__f12_.htm, read 2026-11-13) says: "At the end of analysis, you are asked whether you want to “View results?” Click on OK to see results or Cancel to return to the main window." It also notes: "Command “Analyze” in the “File” menu will become inactive when the model is not edited and “.res” file is available in the directory where the stress model “.mod” is stored." If Analyze is inactive, note it on the run line and go on to the next case.
4. Print the results to a file. The print page (CAEPIPE-PRINT-TO-FILE, https://docs.sstcae.com/users_manual/print__ctrl_p__2.htm) says of the Results tab: "On this tab, check the different results items that you want to print." and of the To File button: "If you want to send all of your selected items (input and results) click on the To File button." and "You can export the data to a .txt file or to a .csv file." Choose the .csv file and save it as `<stem>.print.csv` in the case's folder. Ask for these sections, for every load case (`<case>` stands for the load case's name):

   - `Loads on Anchors in Global Coordinates: <case>`
   - `Loads on Anchors in Local Coordinates: <case>`
   - `Element forces in global coordinates: <case>`
   - `<case>: Loads on restraints`
   - `<case>: Pipe element forces (local)`
   - `<case>: Displacements`

A window-only case's folder holds `<stem>.window-instruction.txt`, the benchmark manifest's text for that run, byte for byte. Follow it step by step in place of the four steps above; this document does not restate it. Its print and its saved model take the names section 5 gives.

A window instruction may end with a paragraph offering, on a condition it states, to make a run's setting in the model of an earlier run of the same case that is still open, instead of importing the run's file. Where its condition holds and you take it, save the model and print the results under that run's name in that run's folder, as the paragraph says, and write on its run line that the setting was made by hand. The files expected back in that folder are the same three.

## 4. The dry run comes first

Part 0 is the dry run. Write what you observe on `RUN_SHEET.md` under part 0, after each colon; nothing in this pack says what you will see.

1. Open a Command Prompt, type the two set-up lines (3.1) and paste the dry run's first case line (cpb-g1-05) alone. Write down answer A.
2. Open that case's print (`<stem>.csv`, or `<stem>.txt`) and compare its section titles with the list in 3.2, step 4. On that case's run line, after "section titles of the print (all present, or which are missing)", write `all present` or the titles that are missing.
3. Paste each other case line of part 0 one line at a time.
4. Paste the batch sample's line (sample-dyn-batch). Write down answer B.
5. In the window sample's folder (`samples/sample-dyn-window`), import `sample-dyn-window.mbf` in the program's window with the import command in 3.2, step 1. Write down answer C. Make checks (i) and (ii) against the stated values for this sample in section 5, and check (iv), and write down each answer. Then save the model as `sample-dyn-window.mod` in that folder, as a real window run saves its model as `<run id>.mod` in its own folder. The Save page (CAEPIPE-SAVE, https://docs.sstcae.com/users_manual/save__ctrl_s_.htm, read 2026-11-13) says: "This command saves model data to a .mod file format." The File Menu page (CAEPIPE-FILE-MENU, https://docs.sstcae.com/users_manual/file_menu_2.htm, read 2026-11-13) lists no command for saving under another name. Then make check (iii): write down the command that saved the imported model; whether the program let you choose the name and the folder; the name and folder the saved file took; whether the program asked anything. Do not save over the imported file. Do not analyse or print. This sample has no line in `BATCH_COMMANDS.txt`.
6. Then go to the export trial's folder (`samples/sample-dyn-export-trial`) and make check (v) there, in that folder only, as the benchmark manifest gives it below this list. Write each answer on that folder's run line. The window sample's folder (`samples/sample-dyn-window`) and every case folder are not opened or touched by this trial.
7. Read the consequences listed under part 0 of `RUN_SHEET.md` and write down which apply.

8. After part 0, copy the working folder back to the Mac, leaving the working folder on Windows as it is. It is listed there by name, size and hash. Wait until you are told the listing is done, whether or not a title was missing, and then go on to part 1.
9. If a section title is missing from the first case's print, write which on that case's run line, and say so when the folder is brought back: what to do about it is decided then, before part 1.

Check (v), from the benchmark manifest:

(v) on a sacrificial duplicate: the folder samples/sample-dyn-export-trial holds sample-dyn-export-trial.mbf, a copy of the same run file as the window sample (cpb-dyn-c-n01-flat--srss-srss); in that folder only:
1. Import sample-dyn-export-trial.mbf: File > Import from the Main window, select the file and click Import. [https://docs.sstcae.com/users_manual/import_mbf.htm ("IMPORT MBF"; CAEPIPE-IMPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
2. From the Layout window, choose File > Export ("To export a model, select the menu command File > Export from the Layout window."). [https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, "EXPORT MBF"; CAEPIPE-EXPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
3. Before touching the dialog, write down the files the folder now holds, by name.
4. In the Export Model dialog, choose "Model batch file (*.mbf)" as the Save as type. [https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, "EXPORT MBF"; CAEPIPE-EXPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
5. Note whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, and whether it displays them before anything is written.
6. If it does, click Export; if not, click Cancel. [https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, "EXPORT MBF"; CAEPIPE-EXPORT-MBF, execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read 2026-11-12]
7. Note the files the folder then holds, by name.
8. Nothing in samples/sample-dyn-window or in any case folder is opened or touched by this trial.

Its consequence, from the benchmark manifest:

(v) the dialog did not let you set the name and the folder, or a file was written before a destination was offered: no run exports at this sitting, and the export steps of every run are skipped and noted

Use the same working folder for the rest of the sitting, and bring it back to the Mac whole at the end.

## 5. Per item

### 1. cpb-g1-05 (benchmark case)

Folder `cases/cpb-g1-05`; model file `cpb-g1-05.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-05` of the pack folder):

- by the batch line: `cpb-g1-05.csv (or cpb-g1-05.txt)`, `cpb-g1-05.mod`, `cpb-g1-05.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-05.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-05
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
section titles of the print (all present, or which are missing): 
```

### 2. cpb-g8-01 (benchmark case)

Folder `cases/cpb-g8-01`; model file `cpb-g8-01.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g8-01` of the pack folder):

- by the batch line: `cpb-g8-01.csv (or cpb-g8-01.txt)`, `cpb-g8-01.mod`, `cpb-g8-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g8-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g8-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 3. cpb-g6-02 (benchmark case)

Folder `cases/cpb-g6-02`; model file `cpb-g6-02.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g6-02` of the pack folder):

- by the batch line: `cpb-g6-02.csv (or cpb-g6-02.txt)`, `cpb-g6-02.mod`, `cpb-g6-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g6-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g6-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 4. sample-dyn-batch (dry-run sample, batch)

Folder `samples/sample-dyn-batch`; model file `sample-dyn-batch.mbf`, layout rows 3; offered: `manual_batch`.

A copy of the model file of the run cpb-dyn-c-n01-flat--srss-srss (SHA-256 239c4758a55c), which carries both added records. It is never screened or compared.

Observe: B. the batch sample (sample-dyn-batch), whose file carries the dynamics options record (OPMISC, with a number of modes) and the spectrum table: whether its print lists modes, and the section titles it holds.

Expected back in this folder (`samples/sample-dyn-batch` of the pack folder):

- by the batch line: `sample-dyn-batch.csv (or sample-dyn-batch.txt)`, `sample-dyn-batch.mod`, `sample-dyn-batch.res`. Which of these the program writes is the dry run's answer A.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: sample-dyn-batch
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 5. sample-dyn-window (dry-run sample, window)

Folder `samples/sample-dyn-window`; model file `sample-dyn-window.mbf`, layout rows 3; offered: `manual_print`.

A copy of the model file of the run cpb-dyn-c-n01-flat--srss-srss (SHA-256 239c4758a55c), which carries both added records. It is never screened or compared.

Stated values: the spectrum DYNFLAT, 2 entries: 0.125, 2500; 1000, 2500; the file's unit codes are 0 for the abscissa and 3 for the ordinate, which the benchmark manifest reads, by inference, as frequency (Hz) and acceleration (mm/sec2). Cutoff Frequency 900 Hz and Number of Modes 2. The vendor's public import page gives the file's cutoff frequency value no unit; Hertz is taken from the analysis page's description of the window's Cutoff Frequency field.

Expected back in this folder (`samples/sample-dyn-window` of the pack folder):

- by the window way: `sample-dyn-window.mod`

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: sample-dyn-window
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
C. after the import in the window sample's folder, whether the import wrote a model file, and where (the export page, CAEPIPE-EXPORT-MBF, says "But note that the existing .mod file will be overwritten. So, you must rename the edited text file before importing it back into CAEPIPE." and does not say in which folder): 
(i) compare the spectrum table the program shows with the stated values, and note agree or not: 
(ii) compare the Dynamics tab's Cutoff Frequency and Number of Modes with the stated values, and note agree or not; if the window shows the file's value in another unit, note that once: 
(iii) write down the command that saved the imported model; whether the program let you choose the name and the folder; the name and folder the saved file took; whether the program asked anything: 
(iv) note what the spectrum options window (Options > Spectrum) offers for axis units and interpolation: 
```

### 6. sample-dyn-export-trial (dry-run sample, export trial)

Folder `samples/sample-dyn-export-trial`; model file `sample-dyn-export-trial.mbf`, layout rows 3; offered: `manual_print`.

A copy of the model file of the run cpb-dyn-c-n01-flat--srss-srss (SHA-256 239c4758a55c), which carries both added records. It is never screened or compared.

Observe: check (v) of the dry run, in this folder only (INSTRUCTIONS.md section 4, step 6).

Expected back in this folder (`samples/sample-dyn-export-trial` of the pack folder):

- by the window way: the model file the import writes, if it writes one (answer C), and `sample-dyn-export-trial.export.mbf` (if check (v) allowed the export)

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: sample-dyn-export-trial
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
(v) step 3, before touching the dialog, the files the folder now holds, by name: 
(v) step 5, whether the dialog lets you set the name sample-dyn-export-trial.export.mbf and this folder, and whether it displays them before anything is written: 
(v) step 6, Export or Cancel ("If it does, click Export; if not, click Cancel"): 
(v) step 7, the files the folder then holds, by name: 
```

### 7. cpb-dyn-c-n08-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--srss-srss`; model file `cpb-dyn-c-n08-flat--srss-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--srss-srss.print.csv`; saved model `cpb-dyn-c-n08-flat--srss-srss.mod`.

Strike order 1, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--srss-srss.print.csv`, `cpb-dyn-c-n08-flat--srss-srss.mod`, `cpb-dyn-c-n08-flat--srss-srss.res`, `cpb-dyn-c-n08-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 8. cpb-dyn-c-n08-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--srss-srss-table-typed`; model file `cpb-dyn-c-n08-flat--srss-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--srss-srss-table-typed.mod`.

Strike order 1, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n08-flat--srss-srss-table-typed.mod`, `cpb-dyn-c-n08-flat--srss-srss-table-typed.res`, `cpb-dyn-c-n08-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 9. cpb-dyn-c-n02-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--srss-srss`; model file `cpb-dyn-c-n02-flat--srss-srss.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--srss-srss.print.csv`; saved model `cpb-dyn-c-n02-flat--srss-srss.mod`.

Strike order 2, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--srss-srss.print.csv`, `cpb-dyn-c-n02-flat--srss-srss.mod`, `cpb-dyn-c-n02-flat--srss-srss.res`, `cpb-dyn-c-n02-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 10. cpb-dyn-c-n02-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--srss-srss-table-typed`; model file `cpb-dyn-c-n02-flat--srss-srss-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--srss-srss-table-typed.mod`.

Strike order 2, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n02-flat--srss-srss-table-typed.mod`, `cpb-dyn-c-n02-flat--srss-srss-table-typed.res`, `cpb-dyn-c-n02-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 11. cpb-dyn-c-n16-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--srss-srss`; model file `cpb-dyn-c-n16-flat--srss-srss.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--srss-srss.print.csv`; saved model `cpb-dyn-c-n16-flat--srss-srss.mod`.

Strike order 3, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--srss-srss.print.csv`, `cpb-dyn-c-n16-flat--srss-srss.mod`, `cpb-dyn-c-n16-flat--srss-srss.res`, `cpb-dyn-c-n16-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 12. cpb-dyn-c-n16-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--srss-srss-table-typed`; model file `cpb-dyn-c-n16-flat--srss-srss-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--srss-srss-table-typed.mod`.

Strike order 3, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n16-flat--srss-srss-table-typed.mod`, `cpb-dyn-c-n16-flat--srss-srss-table-typed.res`, `cpb-dyn-c-n16-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 13. cpb-dyn-c-n08-trunc--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n08-trunc--srss-srss`; model file `cpb-dyn-c-n08-trunc--srss-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-trunc--srss-srss.window-instruction.txt`. Print to `cpb-dyn-c-n08-trunc--srss-srss.print.csv`; saved model `cpb-dyn-c-n08-trunc--srss-srss.mod`.

Strike order 4, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-trunc--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-trunc--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n08-trunc--srss-srss.print.csv`, `cpb-dyn-c-n08-trunc--srss-srss.mod`, `cpb-dyn-c-n08-trunc--srss-srss.res`, `cpb-dyn-c-n08-trunc--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-trunc--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 14. cpb-dyn-c-n08-trunc--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-trunc--srss-srss-table-typed`; model file `cpb-dyn-c-n08-trunc--srss-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-trunc--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-trunc--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n08-trunc--srss-srss-table-typed.mod`.

Strike order 4, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-trunc--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-trunc--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-trunc--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n08-trunc--srss-srss-table-typed.mod`, `cpb-dyn-c-n08-trunc--srss-srss-table-typed.res`, `cpb-dyn-c-n08-trunc--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-trunc--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 15. cpb-dyn-d-n08-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--srss-srss`; model file `cpb-dyn-d-n08-flat--srss-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--srss-srss.print.csv`; saved model `cpb-dyn-d-n08-flat--srss-srss.mod`.

Strike order 5, form A, spectrum table added; its counterpart is cpb-dyn-d-n08-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--srss-srss.print.csv`, `cpb-dyn-d-n08-flat--srss-srss.mod`, `cpb-dyn-d-n08-flat--srss-srss.res`, `cpb-dyn-d-n08-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 16. cpb-dyn-d-n08-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--srss-srss-table-typed`; model file `cpb-dyn-d-n08-flat--srss-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-d-n08-flat--srss-srss-table-typed.mod`.

Strike order 5, form B, spectrum table typed; its counterpart is cpb-dyn-d-n08-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-d-n08-flat--srss-srss-table-typed.mod`, `cpb-dyn-d-n08-flat--srss-srss-table-typed.res`, `cpb-dyn-d-n08-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 17. cpb-dyn-c-n08-two--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-two--srss-srss-table-typed`; model file `cpb-dyn-c-n08-two--srss-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-two--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-two--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n08-two--srss-srss-table-typed.mod`.

Strike order 6, form B, spectrum table typed; declared in this form only.

From the benchmark manifest: "cpb-dyn-c-n08-two is declared in the typed form only (series dynamics-comparable-table-typed, run cpb-dyn-c-n08-two--srss-srss-table-typed). Its spectrum table (DYNTWO, three rows) is carried by no dry-run sample, so the condition on which the form that writes the table may be run could never be met; by design the typed form is this case's only form. It is not a run that was skipped"

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-two--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-two--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n08-two--srss-srss-table-typed.mod`, `cpb-dyn-c-n08-two--srss-srss-table-typed.res`, `cpb-dyn-c-n08-two--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-two--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 18. cpb-dyn-m-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-m-flat--srss-srss`; model file `cpb-dyn-m-flat--srss-srss.mbf`, layout rows 13; offered: `manual_print`.

Window only: follow `cpb-dyn-m-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-m-flat--srss-srss.print.csv`; saved model `cpb-dyn-m-flat--srss-srss.mod`.

Strike order 7, form A, spectrum table added; its counterpart is cpb-dyn-m-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-m-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-m-flat--srss-srss.print.csv`, `cpb-dyn-m-flat--srss-srss.mod`, `cpb-dyn-m-flat--srss-srss.res`, `cpb-dyn-m-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-m-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 19. cpb-dyn-m-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-m-flat--srss-srss-table-typed`; model file `cpb-dyn-m-flat--srss-srss-table-typed.mbf`, layout rows 13; offered: `manual_print`.

Window only: follow `cpb-dyn-m-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-m-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-m-flat--srss-srss-table-typed.mod`.

Strike order 7, form B, spectrum table typed; its counterpart is cpb-dyn-m-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-m-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-m-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-m-flat--srss-srss-table-typed.mod`, `cpb-dyn-m-flat--srss-srss-table-typed.res`, `cpb-dyn-m-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-m-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 20. cpb-dyn-c-n04-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n04-flat--srss-srss`; model file `cpb-dyn-c-n04-flat--srss-srss.mbf`, layout rows 6; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n04-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-c-n04-flat--srss-srss.print.csv`; saved model `cpb-dyn-c-n04-flat--srss-srss.mod`.

Strike order 8, form A, spectrum table added; its counterpart is cpb-dyn-c-n04-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n04-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n04-flat--srss-srss.print.csv`, `cpb-dyn-c-n04-flat--srss-srss.mod`, `cpb-dyn-c-n04-flat--srss-srss.res`, `cpb-dyn-c-n04-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n04-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 21. cpb-dyn-c-n04-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n04-flat--srss-srss-table-typed`; model file `cpb-dyn-c-n04-flat--srss-srss-table-typed.mbf`, layout rows 6; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n04-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n04-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n04-flat--srss-srss-table-typed.mod`.

Strike order 8, form B, spectrum table typed; its counterpart is cpb-dyn-c-n04-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n04-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n04-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n04-flat--srss-srss-table-typed.mod`, `cpb-dyn-c-n04-flat--srss-srss-table-typed.res`, `cpb-dyn-c-n04-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n04-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 22. cpb-dyn-l-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-l-flat--srss-srss`; model file `cpb-dyn-l-flat--srss-srss.mbf`, layout rows 9; offered: `manual_print`.

Window only: follow `cpb-dyn-l-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-l-flat--srss-srss.print.csv`; saved model `cpb-dyn-l-flat--srss-srss.mod`.

Strike order 9, form A, spectrum table added; its counterpart is cpb-dyn-l-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-l-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-l-flat--srss-srss.print.csv`, `cpb-dyn-l-flat--srss-srss.mod`, `cpb-dyn-l-flat--srss-srss.res`, `cpb-dyn-l-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-l-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 23. cpb-dyn-l-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-l-flat--srss-srss-table-typed`; model file `cpb-dyn-l-flat--srss-srss-table-typed.mbf`, layout rows 9; offered: `manual_print`.

Window only: follow `cpb-dyn-l-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-l-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-l-flat--srss-srss-table-typed.mod`.

Strike order 9, form B, spectrum table typed; its counterpart is cpb-dyn-l-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-l-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-l-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-l-flat--srss-srss-table-typed.mod`, `cpb-dyn-l-flat--srss-srss-table-typed.res`, `cpb-dyn-l-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-l-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 24. cpb-dyn-c-n01-flat--srss-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n01-flat--srss-srss`; model file `cpb-dyn-c-n01-flat--srss-srss.mbf`, layout rows 3; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n01-flat--srss-srss.window-instruction.txt`. Print to `cpb-dyn-c-n01-flat--srss-srss.print.csv`; saved model `cpb-dyn-c-n01-flat--srss-srss.mod`.

Strike order 10, form A, spectrum table added; its counterpart is cpb-dyn-c-n01-flat--srss-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n01-flat--srss-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n01-flat--srss-srss.print.csv`, `cpb-dyn-c-n01-flat--srss-srss.mod`, `cpb-dyn-c-n01-flat--srss-srss.res`, `cpb-dyn-c-n01-flat--srss-srss.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n01-flat--srss-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 25. cpb-dyn-c-n01-flat--srss-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n01-flat--srss-srss-table-typed`; model file `cpb-dyn-c-n01-flat--srss-srss-table-typed.mbf`, layout rows 3; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n01-flat--srss-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n01-flat--srss-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n01-flat--srss-srss-table-typed.mod`.

Strike order 10, form B, spectrum table typed; its counterpart is cpb-dyn-c-n01-flat--srss-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n01-flat--srss-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n01-flat--srss-srss-table-typed.print.csv`, `cpb-dyn-c-n01-flat--srss-srss-table-typed.mod`, `cpb-dyn-c-n01-flat--srss-srss-table-typed.res`, `cpb-dyn-c-n01-flat--srss-srss-table-typed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n01-flat--srss-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 26. cpb-dyn-c-n08-flat--abs-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--abs-abs`; model file `cpb-dyn-c-n08-flat--abs-abs.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--abs-abs.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--abs-abs.print.csv`; saved model `cpb-dyn-c-n08-flat--abs-abs.mod`.

Strike order 11, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--abs-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--abs-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--abs-abs.print.csv`, `cpb-dyn-c-n08-flat--abs-abs.mod`, `cpb-dyn-c-n08-flat--abs-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--abs-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 27. cpb-dyn-c-n08-flat--abs-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--abs-abs-table-typed`; model file `cpb-dyn-c-n08-flat--abs-abs-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--abs-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--abs-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--abs-abs-table-typed.mod`.

Strike order 11, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--abs-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--abs-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--abs-abs-table-typed.print.csv`, `cpb-dyn-c-n08-flat--abs-abs-table-typed.mod`, `cpb-dyn-c-n08-flat--abs-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--abs-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 28. cpb-dyn-c-n08-flat--abs-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--abs-srss`; model file `cpb-dyn-c-n08-flat--abs-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--abs-srss.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--abs-srss.print.csv`; saved model `cpb-dyn-c-n08-flat--abs-srss.mod`.

Strike order 12, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--abs-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--abs-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--abs-srss.print.csv`, `cpb-dyn-c-n08-flat--abs-srss.mod`, `cpb-dyn-c-n08-flat--abs-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--abs-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 29. cpb-dyn-c-n08-flat--abs-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--abs-srss-table-typed`; model file `cpb-dyn-c-n08-flat--abs-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--abs-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--abs-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--abs-srss-table-typed.mod`.

Strike order 12, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--abs-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--abs-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--abs-srss-table-typed.print.csv`, `cpb-dyn-c-n08-flat--abs-srss-table-typed.mod`, `cpb-dyn-c-n08-flat--abs-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--abs-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 30. cpb-dyn-c-n08-flat--srss-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--srss-abs`; model file `cpb-dyn-c-n08-flat--srss-abs.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--srss-abs.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--srss-abs.print.csv`; saved model `cpb-dyn-c-n08-flat--srss-abs.mod`.

Strike order 13, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--srss-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--srss-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--srss-abs.print.csv`, `cpb-dyn-c-n08-flat--srss-abs.mod`, `cpb-dyn-c-n08-flat--srss-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--srss-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 31. cpb-dyn-c-n08-flat--srss-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--srss-abs-table-typed`; model file `cpb-dyn-c-n08-flat--srss-abs-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--srss-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--srss-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--srss-abs-table-typed.mod`.

Strike order 13, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--srss-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--srss-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--srss-abs-table-typed.print.csv`, `cpb-dyn-c-n08-flat--srss-abs-table-typed.mod`, `cpb-dyn-c-n08-flat--srss-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--srss-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 32. cpb-dyn-c-n08-flat--closely-spaced-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--closely-spaced-abs`; model file `cpb-dyn-c-n08-flat--closely-spaced-abs.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--closely-spaced-abs.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--closely-spaced-abs.print.csv`; saved model `cpb-dyn-c-n08-flat--closely-spaced-abs.mod`.

Strike order 14, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--closely-spaced-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--closely-spaced-abs.print.csv`, `cpb-dyn-c-n08-flat--closely-spaced-abs.mod`, `cpb-dyn-c-n08-flat--closely-spaced-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--closely-spaced-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 33. cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed`; model file `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.mod`.

Strike order 14, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--closely-spaced-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.print.csv`, `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.mod`, `cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--closely-spaced-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 34. cpb-dyn-c-n08-flat--closely-spaced-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--closely-spaced-srss`; model file `cpb-dyn-c-n08-flat--closely-spaced-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--closely-spaced-srss.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--closely-spaced-srss.print.csv`; saved model `cpb-dyn-c-n08-flat--closely-spaced-srss.mod`.

Strike order 15, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--closely-spaced-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--closely-spaced-srss.print.csv`, `cpb-dyn-c-n08-flat--closely-spaced-srss.mod`, `cpb-dyn-c-n08-flat--closely-spaced-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--closely-spaced-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 35. cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed`; model file `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.mod`.

Strike order 15, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--closely-spaced-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.print.csv`, `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.mod`, `cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--closely-spaced-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 36. cpb-dyn-c-n08-flat--nrl-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--nrl-abs`; model file `cpb-dyn-c-n08-flat--nrl-abs.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--nrl-abs.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--nrl-abs.print.csv`; saved model `cpb-dyn-c-n08-flat--nrl-abs.mod`.

Strike order 16, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--nrl-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--nrl-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--nrl-abs.print.csv`, `cpb-dyn-c-n08-flat--nrl-abs.mod`, `cpb-dyn-c-n08-flat--nrl-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--nrl-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 37. cpb-dyn-c-n08-flat--nrl-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--nrl-abs-table-typed`; model file `cpb-dyn-c-n08-flat--nrl-abs-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--nrl-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--nrl-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--nrl-abs-table-typed.mod`.

Strike order 16, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--nrl-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--nrl-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--nrl-abs-table-typed.print.csv`, `cpb-dyn-c-n08-flat--nrl-abs-table-typed.mod`, `cpb-dyn-c-n08-flat--nrl-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--nrl-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 38. cpb-dyn-c-n08-flat--nrl-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--nrl-srss`; model file `cpb-dyn-c-n08-flat--nrl-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--nrl-srss.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--nrl-srss.print.csv`; saved model `cpb-dyn-c-n08-flat--nrl-srss.mod`.

Strike order 17, form A, spectrum table added; its counterpart is cpb-dyn-c-n08-flat--nrl-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--nrl-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--nrl-srss.print.csv`, `cpb-dyn-c-n08-flat--nrl-srss.mod`, `cpb-dyn-c-n08-flat--nrl-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--nrl-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 39. cpb-dyn-c-n08-flat--nrl-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n08-flat--nrl-srss-table-typed`; model file `cpb-dyn-c-n08-flat--nrl-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n08-flat--nrl-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n08-flat--nrl-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n08-flat--nrl-srss-table-typed.mod`.

Strike order 17, form B, spectrum table typed; its counterpart is cpb-dyn-c-n08-flat--nrl-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n08-flat--nrl-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n08-flat--nrl-srss-table-typed.print.csv`, `cpb-dyn-c-n08-flat--nrl-srss-table-typed.mod`, `cpb-dyn-c-n08-flat--nrl-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n08-flat--nrl-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 40. cpb-dyn-c-n02-flat--abs-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--abs-abs`; model file `cpb-dyn-c-n02-flat--abs-abs.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--abs-abs.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--abs-abs.print.csv`; saved model `cpb-dyn-c-n02-flat--abs-abs.mod`.

Strike order 18, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--abs-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--abs-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--abs-abs.print.csv`, `cpb-dyn-c-n02-flat--abs-abs.mod`, `cpb-dyn-c-n02-flat--abs-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--abs-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 41. cpb-dyn-c-n02-flat--abs-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--abs-abs-table-typed`; model file `cpb-dyn-c-n02-flat--abs-abs-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--abs-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--abs-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--abs-abs-table-typed.mod`.

Strike order 18, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--abs-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--abs-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--abs-abs-table-typed.print.csv`, `cpb-dyn-c-n02-flat--abs-abs-table-typed.mod`, `cpb-dyn-c-n02-flat--abs-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--abs-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 42. cpb-dyn-c-n02-flat--abs-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--abs-srss`; model file `cpb-dyn-c-n02-flat--abs-srss.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--abs-srss.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--abs-srss.print.csv`; saved model `cpb-dyn-c-n02-flat--abs-srss.mod`.

Strike order 19, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--abs-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--abs-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--abs-srss.print.csv`, `cpb-dyn-c-n02-flat--abs-srss.mod`, `cpb-dyn-c-n02-flat--abs-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--abs-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 43. cpb-dyn-c-n02-flat--abs-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--abs-srss-table-typed`; model file `cpb-dyn-c-n02-flat--abs-srss-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--abs-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--abs-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--abs-srss-table-typed.mod`.

Strike order 19, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--abs-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--abs-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--abs-srss-table-typed.print.csv`, `cpb-dyn-c-n02-flat--abs-srss-table-typed.mod`, `cpb-dyn-c-n02-flat--abs-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--abs-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 44. cpb-dyn-c-n02-flat--srss-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--srss-abs`; model file `cpb-dyn-c-n02-flat--srss-abs.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--srss-abs.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--srss-abs.print.csv`; saved model `cpb-dyn-c-n02-flat--srss-abs.mod`.

Strike order 20, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--srss-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--srss-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--srss-abs.print.csv`, `cpb-dyn-c-n02-flat--srss-abs.mod`, `cpb-dyn-c-n02-flat--srss-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--srss-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 45. cpb-dyn-c-n02-flat--srss-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--srss-abs-table-typed`; model file `cpb-dyn-c-n02-flat--srss-abs-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--srss-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--srss-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--srss-abs-table-typed.mod`.

Strike order 20, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--srss-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--srss-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--srss-abs-table-typed.print.csv`, `cpb-dyn-c-n02-flat--srss-abs-table-typed.mod`, `cpb-dyn-c-n02-flat--srss-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--srss-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 46. cpb-dyn-c-n02-flat--closely-spaced-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--closely-spaced-abs`; model file `cpb-dyn-c-n02-flat--closely-spaced-abs.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--closely-spaced-abs.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--closely-spaced-abs.print.csv`; saved model `cpb-dyn-c-n02-flat--closely-spaced-abs.mod`.

Strike order 21, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--closely-spaced-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--closely-spaced-abs.print.csv`, `cpb-dyn-c-n02-flat--closely-spaced-abs.mod`, `cpb-dyn-c-n02-flat--closely-spaced-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--closely-spaced-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 47. cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed`; model file `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.mod`.

Strike order 21, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--closely-spaced-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.print.csv`, `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.mod`, `cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--closely-spaced-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 48. cpb-dyn-c-n02-flat--closely-spaced-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--closely-spaced-srss`; model file `cpb-dyn-c-n02-flat--closely-spaced-srss.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--closely-spaced-srss.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--closely-spaced-srss.print.csv`; saved model `cpb-dyn-c-n02-flat--closely-spaced-srss.mod`.

Strike order 22, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--closely-spaced-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--closely-spaced-srss.print.csv`, `cpb-dyn-c-n02-flat--closely-spaced-srss.mod`, `cpb-dyn-c-n02-flat--closely-spaced-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--closely-spaced-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 49. cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed`; model file `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.mod`.

Strike order 22, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--closely-spaced-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.print.csv`, `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.mod`, `cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--closely-spaced-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 50. cpb-dyn-c-n02-flat--nrl-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--nrl-abs`; model file `cpb-dyn-c-n02-flat--nrl-abs.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--nrl-abs.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--nrl-abs.print.csv`; saved model `cpb-dyn-c-n02-flat--nrl-abs.mod`.

Strike order 23, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--nrl-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--nrl-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--nrl-abs.print.csv`, `cpb-dyn-c-n02-flat--nrl-abs.mod`, `cpb-dyn-c-n02-flat--nrl-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--nrl-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 51. cpb-dyn-c-n02-flat--nrl-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--nrl-abs-table-typed`; model file `cpb-dyn-c-n02-flat--nrl-abs-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--nrl-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--nrl-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--nrl-abs-table-typed.mod`.

Strike order 23, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--nrl-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--nrl-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--nrl-abs-table-typed.print.csv`, `cpb-dyn-c-n02-flat--nrl-abs-table-typed.mod`, `cpb-dyn-c-n02-flat--nrl-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--nrl-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 52. cpb-dyn-c-n02-flat--nrl-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--nrl-srss`; model file `cpb-dyn-c-n02-flat--nrl-srss.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--nrl-srss.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--nrl-srss.print.csv`; saved model `cpb-dyn-c-n02-flat--nrl-srss.mod`.

Strike order 24, form A, spectrum table added; its counterpart is cpb-dyn-c-n02-flat--nrl-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--nrl-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--nrl-srss.print.csv`, `cpb-dyn-c-n02-flat--nrl-srss.mod`, `cpb-dyn-c-n02-flat--nrl-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--nrl-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 53. cpb-dyn-c-n02-flat--nrl-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n02-flat--nrl-srss-table-typed`; model file `cpb-dyn-c-n02-flat--nrl-srss-table-typed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n02-flat--nrl-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n02-flat--nrl-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n02-flat--nrl-srss-table-typed.mod`.

Strike order 24, form B, spectrum table typed; its counterpart is cpb-dyn-c-n02-flat--nrl-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n02-flat--nrl-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n02-flat--nrl-srss-table-typed.print.csv`, `cpb-dyn-c-n02-flat--nrl-srss-table-typed.mod`, `cpb-dyn-c-n02-flat--nrl-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n02-flat--nrl-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 54. cpb-dyn-c-n16-flat--abs-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--abs-abs`; model file `cpb-dyn-c-n16-flat--abs-abs.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--abs-abs.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--abs-abs.print.csv`; saved model `cpb-dyn-c-n16-flat--abs-abs.mod`.

Strike order 25, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--abs-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--abs-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--abs-abs.print.csv`, `cpb-dyn-c-n16-flat--abs-abs.mod`, `cpb-dyn-c-n16-flat--abs-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--abs-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 55. cpb-dyn-c-n16-flat--abs-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--abs-abs-table-typed`; model file `cpb-dyn-c-n16-flat--abs-abs-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--abs-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--abs-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--abs-abs-table-typed.mod`.

Strike order 25, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--abs-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--abs-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--abs-abs-table-typed.print.csv`, `cpb-dyn-c-n16-flat--abs-abs-table-typed.mod`, `cpb-dyn-c-n16-flat--abs-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--abs-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 56. cpb-dyn-c-n16-flat--abs-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--abs-srss`; model file `cpb-dyn-c-n16-flat--abs-srss.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--abs-srss.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--abs-srss.print.csv`; saved model `cpb-dyn-c-n16-flat--abs-srss.mod`.

Strike order 26, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--abs-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--abs-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--abs-srss.print.csv`, `cpb-dyn-c-n16-flat--abs-srss.mod`, `cpb-dyn-c-n16-flat--abs-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--abs-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 57. cpb-dyn-c-n16-flat--abs-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--abs-srss-table-typed`; model file `cpb-dyn-c-n16-flat--abs-srss-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--abs-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--abs-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--abs-srss-table-typed.mod`.

Strike order 26, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--abs-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--abs-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--abs-srss-table-typed.print.csv`, `cpb-dyn-c-n16-flat--abs-srss-table-typed.mod`, `cpb-dyn-c-n16-flat--abs-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--abs-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 58. cpb-dyn-c-n16-flat--srss-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--srss-abs`; model file `cpb-dyn-c-n16-flat--srss-abs.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--srss-abs.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--srss-abs.print.csv`; saved model `cpb-dyn-c-n16-flat--srss-abs.mod`.

Strike order 27, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--srss-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--srss-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--srss-abs.print.csv`, `cpb-dyn-c-n16-flat--srss-abs.mod`, `cpb-dyn-c-n16-flat--srss-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--srss-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 59. cpb-dyn-c-n16-flat--srss-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--srss-abs-table-typed`; model file `cpb-dyn-c-n16-flat--srss-abs-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--srss-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--srss-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--srss-abs-table-typed.mod`.

Strike order 27, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--srss-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--srss-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--srss-abs-table-typed.print.csv`, `cpb-dyn-c-n16-flat--srss-abs-table-typed.mod`, `cpb-dyn-c-n16-flat--srss-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--srss-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 60. cpb-dyn-c-n16-flat--closely-spaced-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--closely-spaced-abs`; model file `cpb-dyn-c-n16-flat--closely-spaced-abs.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--closely-spaced-abs.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--closely-spaced-abs.print.csv`; saved model `cpb-dyn-c-n16-flat--closely-spaced-abs.mod`.

Strike order 28, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--closely-spaced-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--closely-spaced-abs.print.csv`, `cpb-dyn-c-n16-flat--closely-spaced-abs.mod`, `cpb-dyn-c-n16-flat--closely-spaced-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--closely-spaced-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 61. cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed`; model file `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.mod`.

Strike order 28, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--closely-spaced-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.print.csv`, `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.mod`, `cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--closely-spaced-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 62. cpb-dyn-c-n16-flat--closely-spaced-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--closely-spaced-srss`; model file `cpb-dyn-c-n16-flat--closely-spaced-srss.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--closely-spaced-srss.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--closely-spaced-srss.print.csv`; saved model `cpb-dyn-c-n16-flat--closely-spaced-srss.mod`.

Strike order 29, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--closely-spaced-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--closely-spaced-srss.print.csv`, `cpb-dyn-c-n16-flat--closely-spaced-srss.mod`, `cpb-dyn-c-n16-flat--closely-spaced-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--closely-spaced-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 63. cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed`; model file `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.mod`.

Strike order 29, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--closely-spaced-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.print.csv`, `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.mod`, `cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--closely-spaced-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 64. cpb-dyn-c-n16-flat--nrl-abs (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--nrl-abs`; model file `cpb-dyn-c-n16-flat--nrl-abs.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--nrl-abs.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--nrl-abs.print.csv`; saved model `cpb-dyn-c-n16-flat--nrl-abs.mod`.

Strike order 30, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--nrl-abs-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--nrl-abs` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--nrl-abs.print.csv`, `cpb-dyn-c-n16-flat--nrl-abs.mod`, `cpb-dyn-c-n16-flat--nrl-abs.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--nrl-abs
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 65. cpb-dyn-c-n16-flat--nrl-abs-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--nrl-abs-table-typed`; model file `cpb-dyn-c-n16-flat--nrl-abs-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--nrl-abs-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--nrl-abs-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--nrl-abs-table-typed.mod`.

Strike order 30, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--nrl-abs.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--nrl-abs-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--nrl-abs-table-typed.print.csv`, `cpb-dyn-c-n16-flat--nrl-abs-table-typed.mod`, `cpb-dyn-c-n16-flat--nrl-abs-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--nrl-abs-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 66. cpb-dyn-c-n16-flat--nrl-srss (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--nrl-srss`; model file `cpb-dyn-c-n16-flat--nrl-srss.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--nrl-srss.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--nrl-srss.print.csv`; saved model `cpb-dyn-c-n16-flat--nrl-srss.mod`.

Strike order 31, form A, spectrum table added; its counterpart is cpb-dyn-c-n16-flat--nrl-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--nrl-srss` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--nrl-srss.print.csv`, `cpb-dyn-c-n16-flat--nrl-srss.mod`, `cpb-dyn-c-n16-flat--nrl-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--nrl-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 67. cpb-dyn-c-n16-flat--nrl-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-c-n16-flat--nrl-srss-table-typed`; model file `cpb-dyn-c-n16-flat--nrl-srss-table-typed.mbf`, layout rows 18; offered: `manual_print`.

Window only: follow `cpb-dyn-c-n16-flat--nrl-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-c-n16-flat--nrl-srss-table-typed.print.csv`; saved model `cpb-dyn-c-n16-flat--nrl-srss-table-typed.mod`.

Strike order 31, form B, spectrum table typed; its counterpart is cpb-dyn-c-n16-flat--nrl-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-c-n16-flat--nrl-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-c-n16-flat--nrl-srss-table-typed.print.csv`, `cpb-dyn-c-n16-flat--nrl-srss-table-typed.mod`, `cpb-dyn-c-n16-flat--nrl-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-c-n16-flat--nrl-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 68. cpb-dyn-d-n08-flat--abs-srss (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--abs-srss`; model file `cpb-dyn-d-n08-flat--abs-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--abs-srss.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--abs-srss.print.csv`; saved model `cpb-dyn-d-n08-flat--abs-srss.mod`.

Strike order 32, form A, spectrum table added; its counterpart is cpb-dyn-d-n08-flat--abs-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--abs-srss` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--abs-srss.print.csv`, `cpb-dyn-d-n08-flat--abs-srss.mod`, `cpb-dyn-d-n08-flat--abs-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--abs-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 69. cpb-dyn-d-n08-flat--abs-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--abs-srss-table-typed`; model file `cpb-dyn-d-n08-flat--abs-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--abs-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--abs-srss-table-typed.print.csv`; saved model `cpb-dyn-d-n08-flat--abs-srss-table-typed.mod`.

Strike order 32, form B, spectrum table typed; its counterpart is cpb-dyn-d-n08-flat--abs-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--abs-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--abs-srss-table-typed.print.csv`, `cpb-dyn-d-n08-flat--abs-srss-table-typed.mod`, `cpb-dyn-d-n08-flat--abs-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--abs-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 70. cpb-dyn-d-n08-flat--closely-spaced-srss (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--closely-spaced-srss`; model file `cpb-dyn-d-n08-flat--closely-spaced-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--closely-spaced-srss.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--closely-spaced-srss.print.csv`; saved model `cpb-dyn-d-n08-flat--closely-spaced-srss.mod`.

Strike order 33, form A, spectrum table added; its counterpart is cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--closely-spaced-srss` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--closely-spaced-srss.print.csv`, `cpb-dyn-d-n08-flat--closely-spaced-srss.mod`, `cpb-dyn-d-n08-flat--closely-spaced-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--closely-spaced-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 71. cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed`; model file `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.print.csv`; saved model `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.mod`.

Strike order 33, form B, spectrum table typed; its counterpart is cpb-dyn-d-n08-flat--closely-spaced-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.print.csv`, `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.mod`, `cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--closely-spaced-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 72. cpb-dyn-d-n08-flat--nrl-srss (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--nrl-srss`; model file `cpb-dyn-d-n08-flat--nrl-srss.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--nrl-srss.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--nrl-srss.print.csv`; saved model `cpb-dyn-d-n08-flat--nrl-srss.mod`.

Strike order 34, form A, spectrum table added; its counterpart is cpb-dyn-d-n08-flat--nrl-srss-table-typed.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--nrl-srss` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--nrl-srss.print.csv`, `cpb-dyn-d-n08-flat--nrl-srss.mod`, `cpb-dyn-d-n08-flat--nrl-srss.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--nrl-srss
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 73. cpb-dyn-d-n08-flat--nrl-srss-table-typed (held capture, window only)

Folder `cases/cpb-dyn-d-n08-flat--nrl-srss-table-typed`; model file `cpb-dyn-d-n08-flat--nrl-srss-table-typed.mbf`, layout rows 10; offered: `manual_print`.

Window only: follow `cpb-dyn-d-n08-flat--nrl-srss-table-typed.window-instruction.txt`. Print to `cpb-dyn-d-n08-flat--nrl-srss-table-typed.print.csv`; saved model `cpb-dyn-d-n08-flat--nrl-srss-table-typed.mod`.

Strike order 34, form B, spectrum table typed; its counterpart is cpb-dyn-d-n08-flat--nrl-srss.

From the benchmark manifest, what the screening of this run waits for: "a headless modal and response-spectrum entry of the product, which main does not have; until then the capture is held"

Expected back in this folder (`cases/cpb-dyn-d-n08-flat--nrl-srss-table-typed` of the pack folder):

- by the window way: `cpb-dyn-d-n08-flat--nrl-srss-table-typed.print.csv`, `cpb-dyn-d-n08-flat--nrl-srss-table-typed.mod`, `cpb-dyn-d-n08-flat--nrl-srss-table-typed.res`

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-dyn-d-n08-flat--nrl-srss-table-typed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 74. cpb-p1-01--pressure-default (benchmark case)

Folder `cases/cpb-p1-01--pressure-default`; model file `cpb-p1-01--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p1-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p1-01--pressure-default.csv (or cpb-p1-01--pressure-default.txt)`, `cpb-p1-01--pressure-default.mod`, `cpb-p1-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 75. cpb-p1-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p1-01--pressure-elongation-on`; model file `cpb-p1-01--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p1-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p1-01--pressure-elongation-on.csv (or cpb-p1-01--pressure-elongation-on.txt)`, `cpb-p1-01--pressure-elongation-on.mod`, `cpb-p1-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 76. cpb-p1-02--pressure-default (benchmark case)

Folder `cases/cpb-p1-02--pressure-default`; model file `cpb-p1-02--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p1-02--pressure-default` of the pack folder):

- by the batch line: `cpb-p1-02--pressure-default.csv (or cpb-p1-02--pressure-default.txt)`, `cpb-p1-02--pressure-default.mod`, `cpb-p1-02--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-02--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-02--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 77. cpb-p1-02--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p1-02--pressure-elongation-on`; model file `cpb-p1-02--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p1-02--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p1-02--pressure-elongation-on.csv (or cpb-p1-02--pressure-elongation-on.txt)`, `cpb-p1-02--pressure-elongation-on.mod`, `cpb-p1-02--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-02--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-02--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 78. cpb-p1-03--pressure-default (benchmark case)

Folder `cases/cpb-p1-03--pressure-default`; model file `cpb-p1-03--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p1-03--pressure-default` of the pack folder):

- by the batch line: `cpb-p1-03--pressure-default.csv (or cpb-p1-03--pressure-default.txt)`, `cpb-p1-03--pressure-default.mod`, `cpb-p1-03--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-03--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-03--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 79. cpb-p1-03--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p1-03--pressure-elongation-on`; model file `cpb-p1-03--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p1-03--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p1-03--pressure-elongation-on.csv (or cpb-p1-03--pressure-elongation-on.txt)`, `cpb-p1-03--pressure-elongation-on.mod`, `cpb-p1-03--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-03--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-03--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 80. cpb-p1-04--pressure-default (benchmark case)

Folder `cases/cpb-p1-04--pressure-default`; model file `cpb-p1-04--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p1-04--pressure-default` of the pack folder):

- by the batch line: `cpb-p1-04--pressure-default.csv (or cpb-p1-04--pressure-default.txt)`, `cpb-p1-04--pressure-default.mod`, `cpb-p1-04--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-04--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-04--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 81. cpb-p1-04--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p1-04--pressure-elongation-on`; model file `cpb-p1-04--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p1-04--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p1-04--pressure-elongation-on.csv (or cpb-p1-04--pressure-elongation-on.txt)`, `cpb-p1-04--pressure-elongation-on.mod`, `cpb-p1-04--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-04--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-04--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 82. cpb-p1-05--pressure-default (benchmark case)

Folder `cases/cpb-p1-05--pressure-default`; model file `cpb-p1-05--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p1-05--pressure-default` of the pack folder):

- by the batch line: `cpb-p1-05--pressure-default.csv (or cpb-p1-05--pressure-default.txt)`, `cpb-p1-05--pressure-default.mod`, `cpb-p1-05--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-05--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-05--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 83. cpb-p1-05--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p1-05--pressure-elongation-on`; model file `cpb-p1-05--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p1-05--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p1-05--pressure-elongation-on.csv (or cpb-p1-05--pressure-elongation-on.txt)`, `cpb-p1-05--pressure-elongation-on.mod`, `cpb-p1-05--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-05--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-05--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 84. cpb-p1-06--pressure-default (benchmark case)

Folder `cases/cpb-p1-06--pressure-default`; model file `cpb-p1-06--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p1-06--pressure-default` of the pack folder):

- by the batch line: `cpb-p1-06--pressure-default.csv (or cpb-p1-06--pressure-default.txt)`, `cpb-p1-06--pressure-default.mod`, `cpb-p1-06--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-06--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-06--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 85. cpb-p1-06--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p1-06--pressure-elongation-on`; model file `cpb-p1-06--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p1-06--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p1-06--pressure-elongation-on.csv (or cpb-p1-06--pressure-elongation-on.txt)`, `cpb-p1-06--pressure-elongation-on.mod`, `cpb-p1-06--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-06--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-06--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 86. cpb-p2-01--pressure-default (benchmark case)

Folder `cases/cpb-p2-01--pressure-default`; model file `cpb-p2-01--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p2-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p2-01--pressure-default.csv (or cpb-p2-01--pressure-default.txt)`, `cpb-p2-01--pressure-default.mod`, `cpb-p2-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 87. cpb-p2-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p2-01--pressure-elongation-on`; model file `cpb-p2-01--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p2-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p2-01--pressure-elongation-on.csv (or cpb-p2-01--pressure-elongation-on.txt)`, `cpb-p2-01--pressure-elongation-on.mod`, `cpb-p2-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 88. cpb-p2-02--pressure-default (benchmark case)

Folder `cases/cpb-p2-02--pressure-default`; model file `cpb-p2-02--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p2-02--pressure-default` of the pack folder):

- by the batch line: `cpb-p2-02--pressure-default.csv (or cpb-p2-02--pressure-default.txt)`, `cpb-p2-02--pressure-default.mod`, `cpb-p2-02--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-02--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-02--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 89. cpb-p2-02--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p2-02--pressure-elongation-on`; model file `cpb-p2-02--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p2-02--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p2-02--pressure-elongation-on.csv (or cpb-p2-02--pressure-elongation-on.txt)`, `cpb-p2-02--pressure-elongation-on.mod`, `cpb-p2-02--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-02--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-02--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 90. cpb-p2-03--pressure-default (benchmark case)

Folder `cases/cpb-p2-03--pressure-default`; model file `cpb-p2-03--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p2-03--pressure-default` of the pack folder):

- by the batch line: `cpb-p2-03--pressure-default.csv (or cpb-p2-03--pressure-default.txt)`, `cpb-p2-03--pressure-default.mod`, `cpb-p2-03--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-03--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-03--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 91. cpb-p2-03--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p2-03--pressure-elongation-on`; model file `cpb-p2-03--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p2-03--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p2-03--pressure-elongation-on.csv (or cpb-p2-03--pressure-elongation-on.txt)`, `cpb-p2-03--pressure-elongation-on.mod`, `cpb-p2-03--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-03--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-03--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 92. cpb-p2-04--pressure-default (benchmark case)

Folder `cases/cpb-p2-04--pressure-default`; model file `cpb-p2-04--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p2-04--pressure-default` of the pack folder):

- by the batch line: `cpb-p2-04--pressure-default.csv (or cpb-p2-04--pressure-default.txt)`, `cpb-p2-04--pressure-default.mod`, `cpb-p2-04--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-04--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-04--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 93. cpb-p2-04--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p2-04--pressure-elongation-on`; model file `cpb-p2-04--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p2-04--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p2-04--pressure-elongation-on.csv (or cpb-p2-04--pressure-elongation-on.txt)`, `cpb-p2-04--pressure-elongation-on.mod`, `cpb-p2-04--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-04--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-04--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 94. cpb-p3-01--pressure-default (benchmark case)

Folder `cases/cpb-p3-01--pressure-default`; model file `cpb-p3-01--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p3-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p3-01--pressure-default.csv (or cpb-p3-01--pressure-default.txt)`, `cpb-p3-01--pressure-default.mod`, `cpb-p3-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 95. cpb-p3-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p3-01--pressure-elongation-on`; model file `cpb-p3-01--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p3-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p3-01--pressure-elongation-on.csv (or cpb-p3-01--pressure-elongation-on.txt)`, `cpb-p3-01--pressure-elongation-on.mod`, `cpb-p3-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 96. cpb-p3-02--pressure-default (benchmark case)

Folder `cases/cpb-p3-02--pressure-default`; model file `cpb-p3-02--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p3-02--pressure-default` of the pack folder):

- by the batch line: `cpb-p3-02--pressure-default.csv (or cpb-p3-02--pressure-default.txt)`, `cpb-p3-02--pressure-default.mod`, `cpb-p3-02--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-02--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-02--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 97. cpb-p3-02--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p3-02--pressure-elongation-on`; model file `cpb-p3-02--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p3-02--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p3-02--pressure-elongation-on.csv (or cpb-p3-02--pressure-elongation-on.txt)`, `cpb-p3-02--pressure-elongation-on.mod`, `cpb-p3-02--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-02--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-02--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 98. cpb-p3-03--pressure-default (benchmark case)

Folder `cases/cpb-p3-03--pressure-default`; model file `cpb-p3-03--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p3-03--pressure-default` of the pack folder):

- by the batch line: `cpb-p3-03--pressure-default.csv (or cpb-p3-03--pressure-default.txt)`, `cpb-p3-03--pressure-default.mod`, `cpb-p3-03--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-03--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-03--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 99. cpb-p3-03--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p3-03--pressure-elongation-on`; model file `cpb-p3-03--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p3-03--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p3-03--pressure-elongation-on.csv (or cpb-p3-03--pressure-elongation-on.txt)`, `cpb-p3-03--pressure-elongation-on.mod`, `cpb-p3-03--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-03--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-03--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 100. cpb-p3-04--pressure-default (benchmark case)

Folder `cases/cpb-p3-04--pressure-default`; model file `cpb-p3-04--pressure-default.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p3-04--pressure-default` of the pack folder):

- by the batch line: `cpb-p3-04--pressure-default.csv (or cpb-p3-04--pressure-default.txt)`, `cpb-p3-04--pressure-default.mod`, `cpb-p3-04--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-04--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-04--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 101. cpb-p3-04--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p3-04--pressure-elongation-on`; model file `cpb-p3-04--pressure-elongation-on.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p3-04--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p3-04--pressure-elongation-on.csv (or cpb-p3-04--pressure-elongation-on.txt)`, `cpb-p3-04--pressure-elongation-on.mod`, `cpb-p3-04--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-04--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-04--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 102. cpb-p4-01--pressure-default (benchmark case)

Folder `cases/cpb-p4-01--pressure-default`; model file `cpb-p4-01--pressure-default.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p4-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p4-01--pressure-default.csv (or cpb-p4-01--pressure-default.txt)`, `cpb-p4-01--pressure-default.mod`, `cpb-p4-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 103. cpb-p4-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p4-01--pressure-elongation-on`; model file `cpb-p4-01--pressure-elongation-on.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p4-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p4-01--pressure-elongation-on.csv (or cpb-p4-01--pressure-elongation-on.txt)`, `cpb-p4-01--pressure-elongation-on.mod`, `cpb-p4-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 104. cpb-p4-02--pressure-default (benchmark case)

Folder `cases/cpb-p4-02--pressure-default`; model file `cpb-p4-02--pressure-default.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p4-02--pressure-default` of the pack folder):

- by the batch line: `cpb-p4-02--pressure-default.csv (or cpb-p4-02--pressure-default.txt)`, `cpb-p4-02--pressure-default.mod`, `cpb-p4-02--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-02--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-02--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 105. cpb-p4-02--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p4-02--pressure-elongation-on`; model file `cpb-p4-02--pressure-elongation-on.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p4-02--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p4-02--pressure-elongation-on.csv (or cpb-p4-02--pressure-elongation-on.txt)`, `cpb-p4-02--pressure-elongation-on.mod`, `cpb-p4-02--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-02--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-02--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 106. cpb-p4-03--pressure-default (benchmark case)

Folder `cases/cpb-p4-03--pressure-default`; model file `cpb-p4-03--pressure-default.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p4-03--pressure-default` of the pack folder):

- by the batch line: `cpb-p4-03--pressure-default.csv (or cpb-p4-03--pressure-default.txt)`, `cpb-p4-03--pressure-default.mod`, `cpb-p4-03--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-03--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-03--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 107. cpb-p4-03--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p4-03--pressure-elongation-on`; model file `cpb-p4-03--pressure-elongation-on.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p4-03--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p4-03--pressure-elongation-on.csv (or cpb-p4-03--pressure-elongation-on.txt)`, `cpb-p4-03--pressure-elongation-on.mod`, `cpb-p4-03--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-03--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-03--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 108. cpb-p4-04--pressure-default (benchmark case)

Folder `cases/cpb-p4-04--pressure-default`; model file `cpb-p4-04--pressure-default.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p4-04--pressure-default` of the pack folder):

- by the batch line: `cpb-p4-04--pressure-default.csv (or cpb-p4-04--pressure-default.txt)`, `cpb-p4-04--pressure-default.mod`, `cpb-p4-04--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-04--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-04--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 109. cpb-p4-04--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p4-04--pressure-elongation-on`; model file `cpb-p4-04--pressure-elongation-on.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p4-04--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p4-04--pressure-elongation-on.csv (or cpb-p4-04--pressure-elongation-on.txt)`, `cpb-p4-04--pressure-elongation-on.mod`, `cpb-p4-04--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-04--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-04--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 110. cpb-p5-01--pressure-default (benchmark case)

Folder `cases/cpb-p5-01--pressure-default`; model file `cpb-p5-01--pressure-default.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p5-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p5-01--pressure-default.csv (or cpb-p5-01--pressure-default.txt)`, `cpb-p5-01--pressure-default.mod`, `cpb-p5-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p5-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p5-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 111. cpb-p5-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p5-01--pressure-elongation-on`; model file `cpb-p5-01--pressure-elongation-on.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p5-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p5-01--pressure-elongation-on.csv (or cpb-p5-01--pressure-elongation-on.txt)`, `cpb-p5-01--pressure-elongation-on.mod`, `cpb-p5-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p5-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p5-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 112. cpb-p5-02--pressure-default (benchmark case)

Folder `cases/cpb-p5-02--pressure-default`; model file `cpb-p5-02--pressure-default.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p5-02--pressure-default` of the pack folder):

- by the batch line: `cpb-p5-02--pressure-default.csv (or cpb-p5-02--pressure-default.txt)`, `cpb-p5-02--pressure-default.mod`, `cpb-p5-02--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p5-02--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p5-02--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 113. cpb-p5-02--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p5-02--pressure-elongation-on`; model file `cpb-p5-02--pressure-elongation-on.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p5-02--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p5-02--pressure-elongation-on.csv (or cpb-p5-02--pressure-elongation-on.txt)`, `cpb-p5-02--pressure-elongation-on.mod`, `cpb-p5-02--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p5-02--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p5-02--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 114. cpb-p6-01--pressure-default (benchmark case)

Folder `cases/cpb-p6-01--pressure-default`; model file `cpb-p6-01--pressure-default.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p6-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p6-01--pressure-default.csv (or cpb-p6-01--pressure-default.txt)`, `cpb-p6-01--pressure-default.mod`, `cpb-p6-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p6-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p6-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 115. cpb-p6-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p6-01--pressure-elongation-on`; model file `cpb-p6-01--pressure-elongation-on.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p6-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p6-01--pressure-elongation-on.csv (or cpb-p6-01--pressure-elongation-on.txt)`, `cpb-p6-01--pressure-elongation-on.mod`, `cpb-p6-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p6-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p6-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 116. cpb-p7-01--pressure-default (benchmark case)

Folder `cases/cpb-p7-01--pressure-default`; model file `cpb-p7-01--pressure-default.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,0,0,1,0.

Expected back in this folder (`cases/cpb-p7-01--pressure-default` of the pack folder):

- by the batch line: `cpb-p7-01--pressure-default.csv (or cpb-p7-01--pressure-default.txt)`, `cpb-p7-01--pressure-default.mod`, `cpb-p7-01--pressure-default.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p7-01--pressure-default.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p7-01--pressure-default
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 117. cpb-p7-01--pressure-elongation-on (benchmark case)

Folder `cases/cpb-p7-01--pressure-elongation-on`; model file `cpb-p7-01--pressure-elongation-on.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,1.

Expected back in this folder (`cases/cpb-p7-01--pressure-elongation-on` of the pack folder):

- by the batch line: `cpb-p7-01--pressure-elongation-on.csv (or cpb-p7-01--pressure-elongation-on.txt)`, `cpb-p7-01--pressure-elongation-on.mod`, `cpb-p7-01--pressure-elongation-on.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p7-01--pressure-elongation-on.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p7-01--pressure-elongation-on
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 118. cpb-p1-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p1-01--pressure-bourdon-expansion-capture`; model file `cpb-p1-01--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p1-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p1-01--pressure-bourdon-expansion-capture.csv (or cpb-p1-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p1-01--pressure-bourdon-expansion-capture.mod`, `cpb-p1-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 119. cpb-p1-02--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p1-02--pressure-bourdon-expansion-capture`; model file `cpb-p1-02--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p1-02--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p1-02--pressure-bourdon-expansion-capture.csv (or cpb-p1-02--pressure-bourdon-expansion-capture.txt)`, `cpb-p1-02--pressure-bourdon-expansion-capture.mod`, `cpb-p1-02--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-02--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-02--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 120. cpb-p1-03--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p1-03--pressure-bourdon-expansion-capture`; model file `cpb-p1-03--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p1-03--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p1-03--pressure-bourdon-expansion-capture.csv (or cpb-p1-03--pressure-bourdon-expansion-capture.txt)`, `cpb-p1-03--pressure-bourdon-expansion-capture.mod`, `cpb-p1-03--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-03--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-03--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 121. cpb-p1-04--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p1-04--pressure-bourdon-expansion-capture`; model file `cpb-p1-04--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p1-04--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p1-04--pressure-bourdon-expansion-capture.csv (or cpb-p1-04--pressure-bourdon-expansion-capture.txt)`, `cpb-p1-04--pressure-bourdon-expansion-capture.mod`, `cpb-p1-04--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-04--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-04--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 122. cpb-p1-05--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p1-05--pressure-bourdon-expansion-capture`; model file `cpb-p1-05--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p1-05--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p1-05--pressure-bourdon-expansion-capture.csv (or cpb-p1-05--pressure-bourdon-expansion-capture.txt)`, `cpb-p1-05--pressure-bourdon-expansion-capture.mod`, `cpb-p1-05--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-05--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-05--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 123. cpb-p1-06--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p1-06--pressure-bourdon-expansion-capture`; model file `cpb-p1-06--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p1-06--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p1-06--pressure-bourdon-expansion-capture.csv (or cpb-p1-06--pressure-bourdon-expansion-capture.txt)`, `cpb-p1-06--pressure-bourdon-expansion-capture.mod`, `cpb-p1-06--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p1-06--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p1-06--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 124. cpb-p2-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p2-01--pressure-bourdon-expansion-capture`; model file `cpb-p2-01--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p2-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p2-01--pressure-bourdon-expansion-capture.csv (or cpb-p2-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p2-01--pressure-bourdon-expansion-capture.mod`, `cpb-p2-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 125. cpb-p2-02--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p2-02--pressure-bourdon-expansion-capture`; model file `cpb-p2-02--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p2-02--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p2-02--pressure-bourdon-expansion-capture.csv (or cpb-p2-02--pressure-bourdon-expansion-capture.txt)`, `cpb-p2-02--pressure-bourdon-expansion-capture.mod`, `cpb-p2-02--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-02--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-02--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 126. cpb-p2-03--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p2-03--pressure-bourdon-expansion-capture`; model file `cpb-p2-03--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p2-03--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p2-03--pressure-bourdon-expansion-capture.csv (or cpb-p2-03--pressure-bourdon-expansion-capture.txt)`, `cpb-p2-03--pressure-bourdon-expansion-capture.mod`, `cpb-p2-03--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-03--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-03--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 127. cpb-p2-04--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p2-04--pressure-bourdon-expansion-capture`; model file `cpb-p2-04--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p2-04--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p2-04--pressure-bourdon-expansion-capture.csv (or cpb-p2-04--pressure-bourdon-expansion-capture.txt)`, `cpb-p2-04--pressure-bourdon-expansion-capture.mod`, `cpb-p2-04--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p2-04--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p2-04--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 128. cpb-p3-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p3-01--pressure-bourdon-expansion-capture`; model file `cpb-p3-01--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p3-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p3-01--pressure-bourdon-expansion-capture.csv (or cpb-p3-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p3-01--pressure-bourdon-expansion-capture.mod`, `cpb-p3-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 129. cpb-p3-02--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p3-02--pressure-bourdon-expansion-capture`; model file `cpb-p3-02--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p3-02--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p3-02--pressure-bourdon-expansion-capture.csv (or cpb-p3-02--pressure-bourdon-expansion-capture.txt)`, `cpb-p3-02--pressure-bourdon-expansion-capture.mod`, `cpb-p3-02--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-02--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-02--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 130. cpb-p3-03--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p3-03--pressure-bourdon-expansion-capture`; model file `cpb-p3-03--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p3-03--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p3-03--pressure-bourdon-expansion-capture.csv (or cpb-p3-03--pressure-bourdon-expansion-capture.txt)`, `cpb-p3-03--pressure-bourdon-expansion-capture.mod`, `cpb-p3-03--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-03--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-03--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 131. cpb-p3-04--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p3-04--pressure-bourdon-expansion-capture`; model file `cpb-p3-04--pressure-bourdon-expansion-capture.mbf`, layout rows 3; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p3-04--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p3-04--pressure-bourdon-expansion-capture.csv (or cpb-p3-04--pressure-bourdon-expansion-capture.txt)`, `cpb-p3-04--pressure-bourdon-expansion-capture.mod`, `cpb-p3-04--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p3-04--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p3-04--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 132. cpb-p4-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p4-01--pressure-bourdon-expansion-capture`; model file `cpb-p4-01--pressure-bourdon-expansion-capture.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p4-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p4-01--pressure-bourdon-expansion-capture.csv (or cpb-p4-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p4-01--pressure-bourdon-expansion-capture.mod`, `cpb-p4-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 133. cpb-p4-02--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p4-02--pressure-bourdon-expansion-capture`; model file `cpb-p4-02--pressure-bourdon-expansion-capture.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p4-02--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p4-02--pressure-bourdon-expansion-capture.csv (or cpb-p4-02--pressure-bourdon-expansion-capture.txt)`, `cpb-p4-02--pressure-bourdon-expansion-capture.mod`, `cpb-p4-02--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-02--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-02--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 134. cpb-p4-03--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p4-03--pressure-bourdon-expansion-capture`; model file `cpb-p4-03--pressure-bourdon-expansion-capture.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p4-03--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p4-03--pressure-bourdon-expansion-capture.csv (or cpb-p4-03--pressure-bourdon-expansion-capture.txt)`, `cpb-p4-03--pressure-bourdon-expansion-capture.mod`, `cpb-p4-03--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-03--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-03--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 135. cpb-p4-04--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p4-04--pressure-bourdon-expansion-capture`; model file `cpb-p4-04--pressure-bourdon-expansion-capture.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p4-04--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p4-04--pressure-bourdon-expansion-capture.csv (or cpb-p4-04--pressure-bourdon-expansion-capture.txt)`, `cpb-p4-04--pressure-bourdon-expansion-capture.mod`, `cpb-p4-04--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p4-04--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p4-04--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 136. cpb-p5-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p5-01--pressure-bourdon-expansion-capture`; model file `cpb-p5-01--pressure-bourdon-expansion-capture.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p5-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p5-01--pressure-bourdon-expansion-capture.csv (or cpb-p5-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p5-01--pressure-bourdon-expansion-capture.mod`, `cpb-p5-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p5-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p5-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 137. cpb-p5-02--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p5-02--pressure-bourdon-expansion-capture`; model file `cpb-p5-02--pressure-bourdon-expansion-capture.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p5-02--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p5-02--pressure-bourdon-expansion-capture.csv (or cpb-p5-02--pressure-bourdon-expansion-capture.txt)`, `cpb-p5-02--pressure-bourdon-expansion-capture.mod`, `cpb-p5-02--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p5-02--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p5-02--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 138. cpb-p6-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p6-01--pressure-bourdon-expansion-capture`; model file `cpb-p6-01--pressure-bourdon-expansion-capture.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p6-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p6-01--pressure-bourdon-expansion-capture.csv (or cpb-p6-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p6-01--pressure-bourdon-expansion-capture.mod`, `cpb-p6-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p6-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p6-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 139. cpb-p7-01--pressure-bourdon-expansion-capture (sealed capture)

Folder `cases/cpb-p7-01--pressure-bourdon-expansion-capture`; model file `cpb-p7-01--pressure-bourdon-expansion-capture.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Sealed capture: declared capture-only in the benchmark manifest: no SWBPIPE state of the model equals this run.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options 0,1,0,1,0.

Expected back in this folder (`cases/cpb-p7-01--pressure-bourdon-expansion-capture` of the pack folder):

- by the batch line: `cpb-p7-01--pressure-bourdon-expansion-capture.csv (or cpb-p7-01--pressure-bourdon-expansion-capture.txt)`, `cpb-p7-01--pressure-bourdon-expansion-capture.mod`, `cpb-p7-01--pressure-bourdon-expansion-capture.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-p7-01--pressure-bourdon-expansion-capture.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-p7-01--pressure-bourdon-expansion-capture
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 140. cpb-s1-01 (benchmark case)

Folder `cases/cpb-s1-01`; model file `cpb-s1-01.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s1-01` of the pack folder):

- by the batch line: `cpb-s1-01.csv (or cpb-s1-01.txt)`, `cpb-s1-01.mod`, `cpb-s1-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s1-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s1-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 141. cpb-s1-02 (benchmark case)

Folder `cases/cpb-s1-02`; model file `cpb-s1-02.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s1-02` of the pack folder):

- by the batch line: `cpb-s1-02.csv (or cpb-s1-02.txt)`, `cpb-s1-02.mod`, `cpb-s1-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s1-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s1-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 142. cpb-s1-03 (benchmark case)

Folder `cases/cpb-s1-03`; model file `cpb-s1-03.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s1-03` of the pack folder):

- by the batch line: `cpb-s1-03.csv (or cpb-s1-03.txt)`, `cpb-s1-03.mod`, `cpb-s1-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s1-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s1-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 143. cpb-s1-04 (benchmark case)

Folder `cases/cpb-s1-04`; model file `cpb-s1-04.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s1-04` of the pack folder):

- by the batch line: `cpb-s1-04.csv (or cpb-s1-04.txt)`, `cpb-s1-04.mod`, `cpb-s1-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s1-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s1-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 144. cpb-s2-01 (benchmark case)

Folder `cases/cpb-s2-01`; model file `cpb-s2-01.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s2-01` of the pack folder):

- by the batch line: `cpb-s2-01.csv (or cpb-s2-01.txt)`, `cpb-s2-01.mod`, `cpb-s2-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s2-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s2-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 145. cpb-s2-02 (benchmark case)

Folder `cases/cpb-s2-02`; model file `cpb-s2-02.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s2-02` of the pack folder):

- by the batch line: `cpb-s2-02.csv (or cpb-s2-02.txt)`, `cpb-s2-02.mod`, `cpb-s2-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s2-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s2-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 146. cpb-s2-03 (benchmark case)

Folder `cases/cpb-s2-03`; model file `cpb-s2-03.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s2-03` of the pack folder):

- by the batch line: `cpb-s2-03.csv (or cpb-s2-03.txt)`, `cpb-s2-03.mod`, `cpb-s2-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s2-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s2-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 147. cpb-s3-01 (benchmark case)

Folder `cases/cpb-s3-01`; model file `cpb-s3-01.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s3-01` of the pack folder):

- by the batch line: `cpb-s3-01.csv (or cpb-s3-01.txt)`, `cpb-s3-01.mod`, `cpb-s3-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s3-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s3-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 148. cpb-s3-02 (benchmark case)

Folder `cases/cpb-s3-02`; model file `cpb-s3-02.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s3-02` of the pack folder):

- by the batch line: `cpb-s3-02.csv (or cpb-s3-02.txt)`, `cpb-s3-02.mod`, `cpb-s3-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s3-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s3-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 149. cpb-s3-03 (benchmark case)

Folder `cases/cpb-s3-03`; model file `cpb-s3-03.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s3-03` of the pack folder):

- by the batch line: `cpb-s3-03.csv (or cpb-s3-03.txt)`, `cpb-s3-03.mod`, `cpb-s3-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s3-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s3-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 150. cpb-s3-04 (benchmark case)

Folder `cases/cpb-s3-04`; model file `cpb-s3-04.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s3-04` of the pack folder):

- by the batch line: `cpb-s3-04.csv (or cpb-s3-04.txt)`, `cpb-s3-04.mod`, `cpb-s3-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s3-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s3-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 151. cpb-s3-05 (benchmark case)

Folder `cases/cpb-s3-05`; model file `cpb-s3-05.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-s3-05` of the pack folder):

- by the batch line: `cpb-s3-05.csv (or cpb-s3-05.txt)`, `cpb-s3-05.mod`, `cpb-s3-05.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-s3-05.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-s3-05
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 152. cpb-n-01--stop-axial-closed (held capture, window only)

Folder `cases/cpb-n-01--stop-axial-closed`; model file `cpb-n-01--stop-axial-closed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-n-01--stop-axial-closed.window-instruction.txt`. Print to `cpb-n-01--stop-axial-closed.print.csv`; saved model `cpb-n-01--stop-axial-closed.mod`.

From the benchmark manifest, how this run's capture is held: "captured at the sitting and HELD, stored sealed, until SWBPIPE's own result for this case and this load case is published by the product as checks passed; no comparison is formed before then"

From the benchmark manifest, on its one-way support: "a one-way support: its capture is taken at the sitting and held; whether SWBPIPE's result for it may be compared is read from the product at each solve and is recorded nowhere here."

Its hold names the product's gate; `QUALIFICATION.txt` applies.

Expected back in this folder (`cases/cpb-n-01--stop-axial-closed` of the pack folder):

- by the window way: `cpb-n-01--stop-axial-closed.print.csv`, `cpb-n-01--stop-axial-closed.mod`, `cpb-n-01--stop-axial-closed.res`, `cpb-n-01--stop-axial-closed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-n-01--stop-axial-closed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 153. cpb-n-02--rest-lift-off (held capture, window only)

Folder `cases/cpb-n-02--rest-lift-off`; model file `cpb-n-02--rest-lift-off.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-n-02--rest-lift-off.window-instruction.txt`. Print to `cpb-n-02--rest-lift-off.print.csv`; saved model `cpb-n-02--rest-lift-off.mod`.

From the benchmark manifest, how this run's capture is held: "captured at the sitting and HELD, stored sealed, until SWBPIPE's own result for this case and this load case is published by the product as checks passed; no comparison is formed before then"

From the benchmark manifest, on its one-way support: "a one-way support: its capture is taken at the sitting and held; whether SWBPIPE's result for it may be compared is read from the product at each solve and is recorded nowhere here."

Its hold names the product's gate; `QUALIFICATION.txt` applies.

Expected back in this folder (`cases/cpb-n-02--rest-lift-off` of the pack folder):

- by the window way: `cpb-n-02--rest-lift-off.print.csv`, `cpb-n-02--rest-lift-off.mod`, `cpb-n-02--rest-lift-off.res`, `cpb-n-02--rest-lift-off.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-n-02--rest-lift-off
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 154. cpb-n-03--rest-closed (held capture, window only)

Folder `cases/cpb-n-03--rest-closed`; model file `cpb-n-03--rest-closed.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-n-03--rest-closed.window-instruction.txt`. Print to `cpb-n-03--rest-closed.print.csv`; saved model `cpb-n-03--rest-closed.mod`.

From the benchmark manifest, how this run's capture is held: "captured at the sitting and HELD, stored sealed, until SWBPIPE's own result for this case and this load case is published by the product as checks passed; no comparison is formed before then"

From the benchmark manifest, on its one-way support: "a one-way support: its capture is taken at the sitting and held; whether SWBPIPE's result for it may be compared is read from the product at each solve and is recorded nowhere here."

Its hold names the product's gate; `QUALIFICATION.txt` applies.

Expected back in this folder (`cases/cpb-n-03--rest-closed` of the pack folder):

- by the window way: `cpb-n-03--rest-closed.print.csv`, `cpb-n-03--rest-closed.mod`, `cpb-n-03--rest-closed.res`, `cpb-n-03--rest-closed.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-n-03--rest-closed
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 155. cpb-n-04--stop-axial-open (held capture, window only)

Folder `cases/cpb-n-04--stop-axial-open`; model file `cpb-n-04--stop-axial-open.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-n-04--stop-axial-open.window-instruction.txt`. Print to `cpb-n-04--stop-axial-open.print.csv`; saved model `cpb-n-04--stop-axial-open.mod`.

From the benchmark manifest, how this run's capture is held: "captured at the sitting and HELD, stored sealed, until SWBPIPE's own result for this case and this load case is published by the product as checks passed; no comparison is formed before then"

From the benchmark manifest, on its one-way support: "a one-way support: its capture is taken at the sitting and held; whether SWBPIPE's result for it may be compared is read from the product at each solve and is recorded nowhere here."

Its hold names the product's gate; `QUALIFICATION.txt` applies.

Expected back in this folder (`cases/cpb-n-04--stop-axial-open` of the pack folder):

- by the window way: `cpb-n-04--stop-axial-open.print.csv`, `cpb-n-04--stop-axial-open.mod`, `cpb-n-04--stop-axial-open.res`, `cpb-n-04--stop-axial-open.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-n-04--stop-axial-open
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 156. cpb-h-01--constant-support (held capture, window only)

Folder `cases/cpb-h-01--constant-support`; model file `cpb-h-01--constant-support.mbf`, layout rows 4; offered: `manual_print`.

Window only: follow `cpb-h-01--constant-support.window-instruction.txt`. Print to `cpb-h-01--constant-support.print.csv`; saved model `cpb-h-01--constant-support.mod`.

From its model document (`validation/caepipe_benchmark/models/cpb-h-01.json`): "The support's load is an input NOT held equal, named constant_support_load: SWBPIPE states it (3|w|L/8), the target program's window calculates it by its own procedure. SWBPIPE's stated load is never adjusted to what comes back"

From the benchmark manifest, how this run's capture is held: "captured at the sitting and HELD, stored sealed, and not compared, until an independently justified one-parameter series supplying the evidence REQ-019 requires for the input constant_support_load, which no series does yet, as well as SWBPIPE's own result for this case and this load case published by the product as checks passed and the capture's binding to the model actually run; the series's load values are never fitted to the captured result"

Its hold names the product's gate; `QUALIFICATION.txt` applies.

Expected back in this folder (`cases/cpb-h-01--constant-support` of the pack folder):

- by the window way: `cpb-h-01--constant-support.print.csv`, `cpb-h-01--constant-support.mod`, `cpb-h-01--constant-support.res`, `cpb-h-01--constant-support.export.mbf` (if the export was made)

A capture brought back without this evidence may remain unusable for a qualified comparison. Bring back everything in each run's folder, and do not tidy, rename or delete anything.

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-h-01--constant-support
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
export (made, or not made and why): 
```

### 157. cpb-g1-01 (benchmark case)

Folder `cases/cpb-g1-01`; model file `cpb-g1-01.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-01` of the pack folder):

- by the batch line: `cpb-g1-01.csv (or cpb-g1-01.txt)`, `cpb-g1-01.mod`, `cpb-g1-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 158. cpb-g1-02 (benchmark case)

Folder `cases/cpb-g1-02`; model file `cpb-g1-02.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-02` of the pack folder):

- by the batch line: `cpb-g1-02.csv (or cpb-g1-02.txt)`, `cpb-g1-02.mod`, `cpb-g1-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 159. cpb-g1-03 (benchmark case)

Folder `cases/cpb-g1-03`; model file `cpb-g1-03.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-03` of the pack folder):

- by the batch line: `cpb-g1-03.csv (or cpb-g1-03.txt)`, `cpb-g1-03.mod`, `cpb-g1-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 160. cpb-g1-04 (benchmark case)

Folder `cases/cpb-g1-04`; model file `cpb-g1-04.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-04` of the pack folder):

- by the batch line: `cpb-g1-04.csv (or cpb-g1-04.txt)`, `cpb-g1-04.mod`, `cpb-g1-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 161. cpb-g1-06 (benchmark case)

Folder `cases/cpb-g1-06`; model file `cpb-g1-06.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-06` of the pack folder):

- by the batch line: `cpb-g1-06.csv (or cpb-g1-06.txt)`, `cpb-g1-06.mod`, `cpb-g1-06.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-06.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-06
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 162. cpb-g1-07 (benchmark case)

Folder `cases/cpb-g1-07`; model file `cpb-g1-07.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-07` of the pack folder):

- by the batch line: `cpb-g1-07.csv (or cpb-g1-07.txt)`, `cpb-g1-07.mod`, `cpb-g1-07.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-07.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-07
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 163. cpb-g1-10 (benchmark case)

Folder `cases/cpb-g1-10`; model file `cpb-g1-10.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-10` of the pack folder):

- by the batch line: `cpb-g1-10.csv (or cpb-g1-10.txt)`, `cpb-g1-10.mod`, `cpb-g1-10.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-10.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-10
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 164. cpb-g1-08 (benchmark case)

Folder `cases/cpb-g1-08`; model file `cpb-g1-08.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-08` of the pack folder):

- by the batch line: `cpb-g1-08.csv (or cpb-g1-08.txt)`, `cpb-g1-08.mod`, `cpb-g1-08.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-08.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-08
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 165. cpb-g1-09 (benchmark case)

Folder `cases/cpb-g1-09`; model file `cpb-g1-09.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g1-09` of the pack folder):

- by the batch line: `cpb-g1-09.csv (or cpb-g1-09.txt)`, `cpb-g1-09.mod`, `cpb-g1-09.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g1-09.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g1-09
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 166. cpb-g2-01 (benchmark case)

Folder `cases/cpb-g2-01`; model file `cpb-g2-01.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-01` of the pack folder):

- by the batch line: `cpb-g2-01.csv (or cpb-g2-01.txt)`, `cpb-g2-01.mod`, `cpb-g2-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 167. cpb-g2-02 (benchmark case)

Folder `cases/cpb-g2-02`; model file `cpb-g2-02.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-02` of the pack folder):

- by the batch line: `cpb-g2-02.csv (or cpb-g2-02.txt)`, `cpb-g2-02.mod`, `cpb-g2-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 168. cpb-g2-03 (benchmark case)

Folder `cases/cpb-g2-03`; model file `cpb-g2-03.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-03` of the pack folder):

- by the batch line: `cpb-g2-03.csv (or cpb-g2-03.txt)`, `cpb-g2-03.mod`, `cpb-g2-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 169. cpb-g2-04 (benchmark case)

Folder `cases/cpb-g2-04`; model file `cpb-g2-04.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-04` of the pack folder):

- by the batch line: `cpb-g2-04.csv (or cpb-g2-04.txt)`, `cpb-g2-04.mod`, `cpb-g2-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 170. cpb-g2-05 (benchmark case)

Folder `cases/cpb-g2-05`; model file `cpb-g2-05.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-05` of the pack folder):

- by the batch line: `cpb-g2-05.csv (or cpb-g2-05.txt)`, `cpb-g2-05.mod`, `cpb-g2-05.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-05.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-05
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 171. cpb-g2-06 (benchmark case)

Folder `cases/cpb-g2-06`; model file `cpb-g2-06.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-06` of the pack folder):

- by the batch line: `cpb-g2-06.csv (or cpb-g2-06.txt)`, `cpb-g2-06.mod`, `cpb-g2-06.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-06.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-06
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 172. cpb-g2-07 (benchmark case)

Folder `cases/cpb-g2-07`; model file `cpb-g2-07.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-07` of the pack folder):

- by the batch line: `cpb-g2-07.csv (or cpb-g2-07.txt)`, `cpb-g2-07.mod`, `cpb-g2-07.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-07.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-07
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 173. cpb-g2-08 (benchmark case)

Folder `cases/cpb-g2-08`; model file `cpb-g2-08.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-08` of the pack folder):

- by the batch line: `cpb-g2-08.csv (or cpb-g2-08.txt)`, `cpb-g2-08.mod`, `cpb-g2-08.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-08.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-08
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 174. cpb-g2-09 (benchmark case)

Folder `cases/cpb-g2-09`; model file `cpb-g2-09.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-09` of the pack folder):

- by the batch line: `cpb-g2-09.csv (or cpb-g2-09.txt)`, `cpb-g2-09.mod`, `cpb-g2-09.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-09.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-09
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 175. cpb-g2-10 (benchmark case)

Folder `cases/cpb-g2-10`; model file `cpb-g2-10.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-10` of the pack folder):

- by the batch line: `cpb-g2-10.csv (or cpb-g2-10.txt)`, `cpb-g2-10.mod`, `cpb-g2-10.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-10.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-10
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 176. cpb-g2-11 (benchmark case)

Folder `cases/cpb-g2-11`; model file `cpb-g2-11.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-11` of the pack folder):

- by the batch line: `cpb-g2-11.csv (or cpb-g2-11.txt)`, `cpb-g2-11.mod`, `cpb-g2-11.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-11.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-11
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 177. cpb-g2-12 (benchmark case)

Folder `cases/cpb-g2-12`; model file `cpb-g2-12.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-12` of the pack folder):

- by the batch line: `cpb-g2-12.csv (or cpb-g2-12.txt)`, `cpb-g2-12.mod`, `cpb-g2-12.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-12.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-12
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 178. cpb-g2-13 (benchmark case)

Folder `cases/cpb-g2-13`; model file `cpb-g2-13.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-13` of the pack folder):

- by the batch line: `cpb-g2-13.csv (or cpb-g2-13.txt)`, `cpb-g2-13.mod`, `cpb-g2-13.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-13.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-13
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 179. cpb-g2-14 (benchmark case)

Folder `cases/cpb-g2-14`; model file `cpb-g2-14.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-14` of the pack folder):

- by the batch line: `cpb-g2-14.csv (or cpb-g2-14.txt)`, `cpb-g2-14.mod`, `cpb-g2-14.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-14.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-14
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 180. cpb-g2-15 (benchmark case)

Folder `cases/cpb-g2-15`; model file `cpb-g2-15.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-15` of the pack folder):

- by the batch line: `cpb-g2-15.csv (or cpb-g2-15.txt)`, `cpb-g2-15.mod`, `cpb-g2-15.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-15.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-15
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 181. cpb-g2-16 (benchmark case)

Folder `cases/cpb-g2-16`; model file `cpb-g2-16.mbf`, layout rows 5; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g2-16` of the pack folder):

- by the batch line: `cpb-g2-16.csv (or cpb-g2-16.txt)`, `cpb-g2-16.mod`, `cpb-g2-16.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g2-16.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g2-16
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 182. cpb-g3-01 (benchmark case)

Folder `cases/cpb-g3-01`; model file `cpb-g3-01.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-01` of the pack folder):

- by the batch line: `cpb-g3-01.csv (or cpb-g3-01.txt)`, `cpb-g3-01.mod`, `cpb-g3-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 183. cpb-g3-02 (benchmark case)

Folder `cases/cpb-g3-02`; model file `cpb-g3-02.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-02` of the pack folder):

- by the batch line: `cpb-g3-02.csv (or cpb-g3-02.txt)`, `cpb-g3-02.mod`, `cpb-g3-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 184. cpb-g3-03 (benchmark case)

Folder `cases/cpb-g3-03`; model file `cpb-g3-03.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-03` of the pack folder):

- by the batch line: `cpb-g3-03.csv (or cpb-g3-03.txt)`, `cpb-g3-03.mod`, `cpb-g3-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 185. cpb-g3-04 (benchmark case)

Folder `cases/cpb-g3-04`; model file `cpb-g3-04.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-04` of the pack folder):

- by the batch line: `cpb-g3-04.csv (or cpb-g3-04.txt)`, `cpb-g3-04.mod`, `cpb-g3-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 186. cpb-g3-05 (benchmark case)

Folder `cases/cpb-g3-05`; model file `cpb-g3-05.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-05` of the pack folder):

- by the batch line: `cpb-g3-05.csv (or cpb-g3-05.txt)`, `cpb-g3-05.mod`, `cpb-g3-05.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-05.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-05
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 187. cpb-g3-06 (benchmark case)

Folder `cases/cpb-g3-06`; model file `cpb-g3-06.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-06` of the pack folder):

- by the batch line: `cpb-g3-06.csv (or cpb-g3-06.txt)`, `cpb-g3-06.mod`, `cpb-g3-06.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-06.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-06
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 188. cpb-g3-07 (benchmark case)

Folder `cases/cpb-g3-07`; model file `cpb-g3-07.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-07` of the pack folder):

- by the batch line: `cpb-g3-07.csv (or cpb-g3-07.txt)`, `cpb-g3-07.mod`, `cpb-g3-07.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-07.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-07
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 189. cpb-g3-08 (benchmark case)

Folder `cases/cpb-g3-08`; model file `cpb-g3-08.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-08` of the pack folder):

- by the batch line: `cpb-g3-08.csv (or cpb-g3-08.txt)`, `cpb-g3-08.mod`, `cpb-g3-08.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-08.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-08
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 190. cpb-g3-09 (benchmark case)

Folder `cases/cpb-g3-09`; model file `cpb-g3-09.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-09` of the pack folder):

- by the batch line: `cpb-g3-09.csv (or cpb-g3-09.txt)`, `cpb-g3-09.mod`, `cpb-g3-09.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-09.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-09
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 191. cpb-g3-10 (benchmark case)

Folder `cases/cpb-g3-10`; model file `cpb-g3-10.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-10` of the pack folder):

- by the batch line: `cpb-g3-10.csv (or cpb-g3-10.txt)`, `cpb-g3-10.mod`, `cpb-g3-10.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-10.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-10
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 192. cpb-g3-11 (benchmark case)

Folder `cases/cpb-g3-11`; model file `cpb-g3-11.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-11` of the pack folder):

- by the batch line: `cpb-g3-11.csv (or cpb-g3-11.txt)`, `cpb-g3-11.mod`, `cpb-g3-11.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-11.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-11
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 193. cpb-g3-12 (benchmark case)

Folder `cases/cpb-g3-12`; model file `cpb-g3-12.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g3-12` of the pack folder):

- by the batch line: `cpb-g3-12.csv (or cpb-g3-12.txt)`, `cpb-g3-12.mod`, `cpb-g3-12.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g3-12.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g3-12
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 194. cpb-g4-01 (benchmark case)

Folder `cases/cpb-g4-01`; model file `cpb-g4-01.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g4-01` of the pack folder):

- by the batch line: `cpb-g4-01.csv (or cpb-g4-01.txt)`, `cpb-g4-01.mod`, `cpb-g4-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g4-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g4-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 195. cpb-g4-02 (benchmark case)

Folder `cases/cpb-g4-02`; model file `cpb-g4-02.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g4-02` of the pack folder):

- by the batch line: `cpb-g4-02.csv (or cpb-g4-02.txt)`, `cpb-g4-02.mod`, `cpb-g4-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g4-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g4-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 196. cpb-g4-03 (benchmark case)

Folder `cases/cpb-g4-03`; model file `cpb-g4-03.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g4-03` of the pack folder):

- by the batch line: `cpb-g4-03.csv (or cpb-g4-03.txt)`, `cpb-g4-03.mod`, `cpb-g4-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g4-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g4-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 197. cpb-g4-04 (benchmark case)

Folder `cases/cpb-g4-04`; model file `cpb-g4-04.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g4-04` of the pack folder):

- by the batch line: `cpb-g4-04.csv (or cpb-g4-04.txt)`, `cpb-g4-04.mod`, `cpb-g4-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g4-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g4-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 198. cpb-g4-05 (benchmark case)

Folder `cases/cpb-g4-05`; model file `cpb-g4-05.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g4-05` of the pack folder):

- by the batch line: `cpb-g4-05.csv (or cpb-g4-05.txt)`, `cpb-g4-05.mod`, `cpb-g4-05.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g4-05.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g4-05
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 199. cpb-g4-06 (benchmark case)

Folder `cases/cpb-g4-06`; model file `cpb-g4-06.mbf`, layout rows 6; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g4-06` of the pack folder):

- by the batch line: `cpb-g4-06.csv (or cpb-g4-06.txt)`, `cpb-g4-06.mod`, `cpb-g4-06.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g4-06.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g4-06
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 200. cpb-g5-01 (benchmark case)

Folder `cases/cpb-g5-01`; model file `cpb-g5-01.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g5-01` of the pack folder):

- by the batch line: `cpb-g5-01.csv (or cpb-g5-01.txt)`, `cpb-g5-01.mod`, `cpb-g5-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g5-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g5-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 201. cpb-g5-02 (benchmark case)

Folder `cases/cpb-g5-02`; model file `cpb-g5-02.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g5-02` of the pack folder):

- by the batch line: `cpb-g5-02.csv (or cpb-g5-02.txt)`, `cpb-g5-02.mod`, `cpb-g5-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g5-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g5-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 202. cpb-g5-03 (benchmark case)

Folder `cases/cpb-g5-03`; model file `cpb-g5-03.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g5-03` of the pack folder):

- by the batch line: `cpb-g5-03.csv (or cpb-g5-03.txt)`, `cpb-g5-03.mod`, `cpb-g5-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g5-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g5-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 203. cpb-g5-04 (benchmark case)

Folder `cases/cpb-g5-04`; model file `cpb-g5-04.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g5-04` of the pack folder):

- by the batch line: `cpb-g5-04.csv (or cpb-g5-04.txt)`, `cpb-g5-04.mod`, `cpb-g5-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g5-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g5-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 204. cpb-g5-05 (benchmark case)

Folder `cases/cpb-g5-05`; model file `cpb-g5-05.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g5-05` of the pack folder):

- by the batch line: `cpb-g5-05.csv (or cpb-g5-05.txt)`, `cpb-g5-05.mod`, `cpb-g5-05.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g5-05.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g5-05
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 205. cpb-g5-06 (benchmark case)

Folder `cases/cpb-g5-06`; model file `cpb-g5-06.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g5-06` of the pack folder):

- by the batch line: `cpb-g5-06.csv (or cpb-g5-06.txt)`, `cpb-g5-06.mod`, `cpb-g5-06.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g5-06.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g5-06
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 206. cpb-g6-01 (benchmark case)

Folder `cases/cpb-g6-01`; model file `cpb-g6-01.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g6-01` of the pack folder):

- by the batch line: `cpb-g6-01.csv (or cpb-g6-01.txt)`, `cpb-g6-01.mod`, `cpb-g6-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g6-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g6-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 207. cpb-g6-03 (benchmark case)

Folder `cases/cpb-g6-03`; model file `cpb-g6-03.mbf`, layout rows 7; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g6-03` of the pack folder):

- by the batch line: `cpb-g6-03.csv (or cpb-g6-03.txt)`, `cpb-g6-03.mod`, `cpb-g6-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g6-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g6-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 208. cpb-g7-01 (benchmark case)

Folder `cases/cpb-g7-01`; model file `cpb-g7-01.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g7-01` of the pack folder):

- by the batch line: `cpb-g7-01.csv (or cpb-g7-01.txt)`, `cpb-g7-01.mod`, `cpb-g7-01.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g7-01.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g7-01
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 209. cpb-g7-02 (benchmark case)

Folder `cases/cpb-g7-02`; model file `cpb-g7-02.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g7-02` of the pack folder):

- by the batch line: `cpb-g7-02.csv (or cpb-g7-02.txt)`, `cpb-g7-02.mod`, `cpb-g7-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g7-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g7-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 210. cpb-g7-03 (benchmark case)

Folder `cases/cpb-g7-03`; model file `cpb-g7-03.mbf`, layout rows 8; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g7-03` of the pack folder):

- by the batch line: `cpb-g7-03.csv (or cpb-g7-03.txt)`, `cpb-g7-03.mod`, `cpb-g7-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g7-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g7-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 211. cpb-g8-02 (benchmark case)

Folder `cases/cpb-g8-02`; model file `cpb-g8-02.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g8-02` of the pack folder):

- by the batch line: `cpb-g8-02.csv (or cpb-g8-02.txt)`, `cpb-g8-02.mod`, `cpb-g8-02.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g8-02.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g8-02
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 212. cpb-g8-03 (benchmark case)

Folder `cases/cpb-g8-03`; model file `cpb-g8-03.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g8-03` of the pack folder):

- by the batch line: `cpb-g8-03.csv (or cpb-g8-03.txt)`, `cpb-g8-03.mod`, `cpb-g8-03.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g8-03.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g8-03
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

### 213. cpb-g8-04 (benchmark case)

Folder `cases/cpb-g8-04`; model file `cpb-g8-04.mbf`, layout rows 4; offered: `manual_batch` and `manual_print`.

Settings to check in the window (3.2, step 2): piping code B311-80; B31J off (OPCODE 0,0,0); reference temperature 20 C; units SI; vertical axis Y; gravity 9.80665 m/s2; pressure options no OPPRES record written.

Expected back in this folder (`cases/cpb-g8-04` of the pack folder):

- by the batch line: `cpb-g8-04.csv (or cpb-g8-04.txt)`, `cpb-g8-04.mod`, `cpb-g8-04.res`. Which of these the program writes is the dry run's answer A.
- by the window way, only if the batch line left no results: `cpb-g8-04.print.csv`, besides the model and results files named above

Run line (`run_line.txt`):

```
SWBPIPE sitting run line: cpb-g8-04
Fill in after each colon. Do not change the words before the colon.
date: 
run (yes, or not run and why): 
route used (manual_batch or manual_print): 
CAEPIPE version as displayed: 
anything unusual: 
```

## Sources

- CAEPIPE-BATCH: https://docs.sstcae.com/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm
- CAEPIPE-IMPORT-MBF: https://docs.sstcae.com/users_manual/import_mbf.htm
- CAEPIPE-EXPORT-MBF: https://docs.sstcae.com/users_manual/export_mbf.htm
- CAEPIPE-PRINT-TO-FILE: https://docs.sstcae.com/users_manual/print__ctrl_p__2.htm
- CAEPIPE-ANALYZE: https://docs.sstcae.com/users_manual/analyze__f12_.htm
- CAEPIPE-SAVE: https://docs.sstcae.com/users_manual/save__ctrl_s_.htm
- CAEPIPE-FILE-MENU: https://docs.sstcae.com/users_manual/file_menu_2.htm
- MS-START: https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/start
