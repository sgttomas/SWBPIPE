"""Tranche 3, part two: cases entered by the owner's hand at the target program's window (DEL-17-05; the "Benchmark
method" section of docs/RELEASE_TARGET_v1.0.md: model families from straight spans to nonlinear supports and hangers).

The MBF writer refuses the members these cases hold (one-way and other nonlinear supports, hangers: MBF-SI-SUPPORT-
UNSUPPORTED), and no support or load record is ever added to a written file. So each case is one SWBPIPE model that
holds the member, and a WRITTEN BASE: the same model with exactly that member left out. The tracked document's `model`
is the base, which the writer writes with no refusal; its MBF is the case's model file and its hash is recorded.
`hand_entry_model` forms the model with the member by the stated rule: the base with the member's support records
appended to model.supports, and nothing else changed. Each case declares one run of kind hand_entry: an instruction a
person at the window can follow, naming the one thing to enter after importing the base, each field with its value,
its unit and its sign convention, each cited to the vendor's public page that states it, a save under the run id
before analysing by the command noted at the dry run (DRY_RUN), and a print to file under the run id. Nothing on the loading path (run_pilot.load_benchmark,
benchmark_models.series_document, run_mbf_text) calls hand_entry_model: what the writer is given is the base.

Only fields whose unit and sign convention a saved public page states for the window are used. The saved pages state
no window length or stiffness unit for a limit stop's limits or a user hanger's spring rate, so every case here enters
values that need no unit: a zero limit, blank limits, direction components, the Vertical preset, a count of hangers.
The cases left out, and the field and page that left each out, are in LEFT_OUT.

Third values are textbook statics and Euler-Bernoulli beam theory, derived in this text: an anchored bar of axial
stiffness E A and length L, uniformly heated by dT, against a one-way stop at its far end with gap g closes when
alpha dT L > g and then carries E A (alpha dT L - g)/L (here g = 0, so E A alpha dT), and carries nothing while open;
a cantilever with a prop at its tip under uniform load w carries 3 w L/8 at the prop (Euler-Bernoulli, ideal supports)
and 5 w L/8 and w L^2/8 at the anchor; a tip load P applied at the prop itself goes wholly into a closed prop, so an
upward P larger than 3|w|L/8 opens a one-way prop and the cantilever is free. Nothing here runs either program or
records a result of either.
"""

from __future__ import annotations

import copy
import hashlib
from typing import Any

from tools.caepipe_correlation import benchmark_models as bm
from tools.caepipe_correlation.benchmark_designed import DESIGNED_MATERIAL, designed_od, designed_wall
from tools.caepipe_correlation.benchmark_models import (
    EXPANSION, OPERATING, SHEAR, SUSTAINED, Builder, closed, section_properties,
)

DATE_READ = "2026-11-12"
RIGID = "rigid_support_stiffness"
PROVENANCE = ("invented CAEPIPE benchmark input (DEL-17-05, tranche 3 part two, entered by hand); seeded generator; "
              "not library, component, catalog, standard or code-rule data")


def _page(page: str, section: str) -> str:
    return f"https://docs.sstcae.com/tech_manual/{page}.htm (technical reference, \"{section}\"), read {DATE_READ}"


LIMIT_STOP = _page("limit_stop", "Limit Stop")
CONSTANT_SUPPORT = _page("constant_support", "Constant Support")
IMPORT = (f"https://docs.sstcae.com/users_manual/import_mbf.htm (\"IMPORT MBF\"; CAEPIPE-IMPORT-MBF, "
          f"execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read {DATE_READ}")
HELD = ("captured at the sitting and HELD, stored sealed, until SWBPIPE's own result for this case and this load case is "
        "published by the product as checks passed; no comparison is formed before then")
# The hold of a case whose named input no one-parameter series carries (REQ-019): cpb-h-01's constant_support_load.
SERIES_WAIT = ("an independently justified one-parameter series supplying the evidence REQ-019 requires for the input "
               "constant_support_load, which no series does yet")
HELD_UNTIL_SERIES = ("captured at the sitting and HELD, stored sealed, and not compared, until " + SERIES_WAIT + ", as "
                     "well as SWBPIPE's own result for this case and this load case published by the product as checks "
                     "passed and the capture's binding to the model actually run; the series's load values are never "
                     "fitted to the captured result")
# The sentence every one-way case carries (the project's Agent 0's words, 2026-11-12); it is the only text here about
# whether SWBPIPE's result for such a case may be compared.
ONE_WAY = ("a one-way support: its capture is taken at the sitting and held; whether SWBPIPE's result for it may be "
           "compared is read from the product at each solve and is recorded nowhere here.")
PRINT_PAGE = f"https://docs.sstcae.com/users_manual/print__ctrl_p__2.htm (user's manual, \"Print (Ctrl+P)\"), read {DATE_READ}"
ANALYSIS_PAGE = (f"https://docs.sstcae.com/users_manual/analysis.htm (user's manual, \"Analysis\"), read {DATE_READ}")
DATE_READ_2 = "2026-11-13"
ANALYZE_PAGE = (f"https://docs.sstcae.com/users_manual/analyze__f12_.htm (user's manual, \"Analyze (F12)\"), read "
                f"{DATE_READ_2}")
SAVE_PAGE = f"https://docs.sstcae.com/users_manual/save__ctrl_s_.htm (user's manual, \"Save (Ctrl+S)\"), read {DATE_READ_2}"
FILE_MENU_PAGE = f"https://docs.sstcae.com/users_manual/file_menu_2.htm (user's manual, \"File Menu\"), read {DATE_READ_2}"
# The Analyze page's words, quoted exactly (users_manual/analyze__f12_.htm); a quotation, not a prediction.
ANALYZE_QUOTE = ("At the end of analysis, you are asked whether you want to \u201cView results?\u201d Click on OK to see "
                 "results or Cancel to return to the main window.")
SAVE_QUOTE = "This command saves model data to a .mod file format."
# The dry run's checks, declared for the pack to carry and print (the project's Agent 0, 2026-11-12). Things to observe;
# nothing here predicts any of them.
DRY_RUN = {
    "sample": "the dry-run sample, which carries the added records (the OPMISC dynamics options record and the SPCMS table)",
    "checks": [
        "(i) compare the spectrum table the program shows with the stated values, and note agree or not",
        ("(ii) compare the Dynamics tab's Cutoff Frequency and Number of Modes with the stated values, and note agree or "
         "not; if the window shows the file's value in another unit, note that once"),
        ("(iii) write down the command that saved the imported model; whether the program let you choose the name and "
         "the folder; the name and folder the saved file took; whether the program asked anything"),
        "(iv) note what the spectrum options window (Options > Spectrum) offers for axis units and interpolation",
    ],
    "consequences": [
        "(i) not agreeing, or not made: form B (spectrum table typed) for every dynamics run",
        ("(ii) another unit: the cutoff correction applies to whichever single form of a pair is run; the owner sets "
         "the stated value only if a public page states the conversion, and otherwise skips the run and notes it"),
        ("(iii) the model cannot be saved as <run id>.mod in the run's folder: every hand-entry and every dynamics run "
         "is skipped and noted"),
    ],
    "save_command": ("a run's model file is <run id>.mbf and its saved model <run id>.mod in the same folder, so no "
                     "other name is needed; the Save page says \"" + SAVE_QUOTE + "\" (" + SAVE_PAGE + "), and the File "
                     "Menu page documents Save and no command for saving a model under another name (" + FILE_MENU_PAGE
                     + "); so the command, whether it lets the person choose the name and folder, and the name and "
                     "folder the saved file takes are observed at the dry run (check iii); every instruction refers to "
                     "that command"),
}


def save_sentence(run_id: str) -> str:
    """The save sentence of every window instruction, exactly as the project's Agent 0 decided it (2026-11-13)."""
    return (f"Save the model as {run_id}.mod in this folder by the command you noted at the dry run. Do not save over "
            f"the imported file. If the program will not save under that name, skip this run and note it.")


Step = tuple[str, tuple[str, ...]]


def numbered(steps: list[Step], closing: Step | None = None, closing_steps: list[Step] | None = None) -> str:
    """A window instruction as one text: numbered steps, one action per step, each step's page citations in brackets
    at its end; then, optionally, an unnumbered closing condition laid out the same way, followed by its own numbered
    steps from 1; plain line breaks, no markup."""
    def line(text: str, pages: tuple[str, ...]) -> str:
        return text + (f" [{'; '.join(pages)}]" if pages else "")

    lines = [f"{index}. {line(*step)}" for index, step in enumerate(steps, 1)]
    if closing:
        lines.append(line(*closing))
        lines += [f"{index}. {line(*step)}" for index, step in enumerate(closing_steps or [], 1)]
    return "\n".join(lines)


# What a skip means: no replacement, and the next item. A dynamics run goes on to the next strike number; a hand-entry
# run to the next case. (Form A's start checks alone send the owner to the counterpart; benchmark_tranche3.)
# A run of a pair ("pair") also says that its counterpart is not run in its place: only form A's named initial checks
# lead to the counterpart (benchmark_tranche3.PAIR_RULE).
NEXT_ITEM = {"dynamics": "the next strike number", "pair": "the next strike number", "hand": "the next case"}
NOT_COUNTERPART = {"pair": " and not its counterpart either"}


def skip(kind: str) -> str:
    """The clause every skip carries: no replacement, noted, and the next item."""
    return (f"skip this run, run nothing in its place{NOT_COUNTERPART.get(kind, '')}, note it on the run line and go "
            f"on to {NEXT_ITEM[kind]}")


def then_next(kind: str) -> str:
    """The next action after a settled sentence that already says to skip and note; conditional on the skip, in the
    words the independent bounded reading proposed, so that it cannot be read as following a successful step."""
    return f"If you skip this run, run nothing in its place{NOT_COUNTERPART.get(kind, '')}; go on to {NEXT_ITEM[kind]}."


def skip_rule(kind: str) -> str:
    """The rule for a field the window does not offer as its page describes; it comes before the optional export."""
    return f"If the window does not offer a field as the page describes it, do not enter an equivalent; {skip(kind)}."


# The Analyze page's sentence on when the command is inactive, quoted exactly (users_manual/analyze__f12_.htm).
ANALYZE_INACTIVE_QUOTE = ("Command “Analyze” in the “File” menu will become inactive when the model "
                          "is not edited and “.res” file is available in the directory where the stress model "
                          "“.mod” is stored.")


def analyze_and_print_steps(run_id: str, kind: str) -> list[Step]:
    """Analyse by File > Analyze (F12), quoting its page, then print to file under the run id, one action per step."""
    return [
        (f"Then analyze the model by File > Analyze (F12); the page says: \"{ANALYZE_QUOTE}\" If the command is "
         f"inactive (\"{ANALYZE_INACTIVE_QUOTE}\"), {skip(kind)}.", (ANALYZE_PAGE,)),
        ("In the Results window, open Print: File > Print (Ctrl+P).", (PRINT_PAGE,)),
        ("Click All.", (PRINT_PAGE,)),
        ("Click To File.", (PRINT_PAGE,)),
        (f"Save the .csv file as {run_id}.print.csv.", (PRINT_PAGE,)),
    ]


def finish_steps(run_id: str, kind: str, export_lost: str | None = None) -> list[Step]:
    """The end of every window instruction: save under the run id by the command observed at the dry run (DRY_RUN; the
    settled sentence, then its next action), analyse, print to file under the run id, the skip rule, then, for a run
    that declares one, the optional export of its saved model (EXPORT_STATEMENT)."""
    return [(f"{save_sentence(run_id)} {then_next(kind)}", ()), *analyze_and_print_steps(run_id, kind),
            (skip_rule(kind), ()), *(export_steps(run_id, export_lost) if export_lost else [])]


# -- the optional export of a run's saved model (the SWBPIPE Agent 0, 2026-11-13) -----------------------------------
EXPORT_PAGE = (f"https://docs.sstcae.com/users_manual/export_mbf.htm (user's manual, \"EXPORT MBF\"; CAEPIPE-EXPORT-MBF, "
               f"execution/PKG-17/DEL-17-01/Source_Basis_Register.md), read {DATE_READ}")
# The Export MBF page's words, quoted exactly (users_manual/export_mbf.htm).
EXPORT_QUOTE_MENU = "To export a model, select the menu command File > Export from the Layout window."
EXPORT_QUOTE_TYPE = ("The Export Model dialog is shown. Selecting the “Save as type” option as “Model batch "
                     "file (*.mbf)” will export the stress model in CAEPIPE latest version.")
EXPORT_QUOTE_BUTTON = "Once done, click on Export button to write to an .mbf file."
EXPORT_NOT_MADE = "export not made"
# The dry run's sacrificial duplicate: the pack author makes this folder; it holds a copy of the same run file as the
# window sample (WINDOW_SAMPLE_RUN), so that the check touches neither that sample nor any case folder.
EXPORT_TRIAL_FOLDER = "samples/sample-dyn-export-trial"
EXPORT_TRIAL_FILE = "sample-dyn-export-trial.mbf"
WINDOW_SAMPLE_FOLDER = "samples/sample-dyn-window"
WINDOW_SAMPLE_RUN = "cpb-dyn-c-n01-flat--srss-srss"
EXPORT_STATEMENT = (
    "an export is a model batch file the program writes, at the owner's choice, from a run's saved model (<run id>.mod) "
    "as the program holds it after that run's instruction, including what was entered or set by hand; it is not the "
    "pack's model file. Each export is private, like every returned file. The exports are material for later "
    "interoperability work: they put on file how the program itself writes records SWBPIPE's writer does not write "
    "today. They do not demonstrate exit criterion 6's round trip in the desktop, and they do not make any family the "
    "product refuses become supported. Nothing of SWBPIPE's reads an export in this change")


def export_name(run_id: str) -> str:
    return f"{run_id}.export.mbf"


def export_binding(run_id: str) -> str:
    """What the export records, bound to this run's saved model."""
    return (f"The file it writes records this run's saved model, {run_id}.mod, as the program holds it after this "
            f"instruction's steps, including what was entered or set by hand; it is not the pack's model file.")


def export_steps(run_id: str, lost_if_dropped: str) -> list[Step]:
    """The optional export, in numbered steps: its condition, binding and loss, the menu, the type, the name, the
    guard. A control not offered as described ends the export, not the run."""
    name = export_name(run_id)
    return [
        (f"Optional: the next four steps export this run's saved model, and you may drop them; if you do, write "
         f"\"{EXPORT_NOT_MADE}\" on the run line. If you drop it, what is lost is {lost_if_dropped}. Take them only if "
         f"the dry run's check (v) found that the export dialog lets you set the file's name and folder. "
         f"{export_binding(run_id)} If any control in these steps is not offered as described, click Cancel if a dialog "
         f"is open, write \"{EXPORT_NOT_MADE}\" and why on the run line; the run is complete.", ()),
        (f"From the Layout window, choose File > Export (\"{EXPORT_QUOTE_MENU}\").", (EXPORT_PAGE,)),
        (f"In the Export Model dialog, choose \"Model batch file (*.mbf)\" as the Save as type (\"{EXPORT_QUOTE_TYPE}\").",
         (EXPORT_PAGE,)),
        (f"Set the name {name} in this run's folder.", ()),
        (f"Click Export (\"{EXPORT_QUOTE_BUTTON}\") only if the dialog displays exactly the name {name} in this run's "
         f"folder and no file of that name is already there. If it does not, or if anything would be written over, "
         f"click Cancel and write \"{EXPORT_NOT_MADE}\" and why on the run line; the run is complete.", (EXPORT_PAGE,)),
    ]


def export_declaration(run_id: str, lost_if_dropped: str) -> dict[str, Any]:
    """The manifest's declaration of a run's optional export."""
    return {"file": export_name(run_id), "optional": True, "if_dropped": f"\"{EXPORT_NOT_MADE}\" on the run line",
            "condition": "the dry run's check (v) found that the export dialog lets the owner set the file's name and "
                         "folder", "records": export_binding(run_id), "lost_if_dropped": lost_if_dropped}


EXPORT_LOST_HAND = {"N": "the program's own writing of the support record entered by hand (a limit stop)",
                    "H": "the program's own writing of the support record entered by hand (a constant support)"}
DRY_RUN["checks"].append(
    f"(v) on a sacrificial duplicate: the folder {EXPORT_TRIAL_FOLDER} holds {EXPORT_TRIAL_FILE}, a copy of the same run "
    f"file as the window sample ({WINDOW_SAMPLE_RUN}); in that folder only:\n" + numbered([
        (f"Import {EXPORT_TRIAL_FILE}: File > Import from the Main window, select the file and click Import.", (IMPORT,)),
        (f"From the Layout window, choose File > Export (\"{EXPORT_QUOTE_MENU}\").", (EXPORT_PAGE,)),
        ("Before touching the dialog, write down the files the folder now holds, by name.", ()),
        ("In the Export Model dialog, choose \"Model batch file (*.mbf)\" as the Save as type.", (EXPORT_PAGE,)),
        (f"Note whether the dialog lets you set the name {EXPORT_TRIAL_FILE[:-4]}.export.mbf and this folder, and "
         f"whether it displays them before anything is written.", ()),
        ("If it does, click Export; if not, click Cancel.", (EXPORT_PAGE,)),
        ("Note the files the folder then holds, by name.", ()),
        (f"Nothing in {WINDOW_SAMPLE_FOLDER} or in any case folder is opened or touched by this check.", ()),
    ]))
DRY_RUN["consequences"].append(
    "(v) the dialog did not let you set the name and the folder, or a file was written before a destination was "
    "offered: no run exports at this sitting, and the export steps of every run are skipped and noted")
DRY_RUN["export_trial"] = {"folder": EXPORT_TRIAL_FOLDER, "file": EXPORT_TRIAL_FILE, "copy_of_run": WINDOW_SAMPLE_RUN,
                           "made_by": "the pack author", "untouched": [WINDOW_SAMPLE_FOLDER, "every case folder"]}


# Designed values (benchmark_designed.py): OD 190 mm by the rule, D/t 25, a span of 3.0 m, a temperature change of 100 K.
OD, DT, SPAN, DELTA_T = designed_od(190.0), 25.0, 3.0, 100.0
WALL = designed_wall(OD, DT)
# The input of cpb-h-01 not held equal: SWBPIPE's member states the constant support's load; the target program's
# window calculates that load itself (its public Constant Support page).
CONSTANT_LOAD = "constant_support_load"

LEFT_OUT = [
    {"case": "a limit stop whose gap closes under thermal growth, with a gap greater than zero",
     "field": "Upper limit / Lower limit (the gap)", "page": LIMIT_STOP,
     "why": ("the page states the limits as gaps measured from the undeflected node and their sign convention, but not "
             "the unit the window takes them in for an SI model (its examples are in inches); the one-way stop "
             "(cpb-n-01) is kept, since zero needs no unit")},
    {"case": "friction at a support", "field": "Friction coefficient (stated, unitless); the load path", "page": LIMIT_STOP,
     "why": ("the friction coefficient is stated and needs no unit, but the case needs the target program's thermal-case "
             "option (thermal as operating minus sustained, or solved alone; https://docs.sstcae.com/users_manual/"
             "analysis.htm states both and not which is set by default for an imported file) and a SWBPIPE state "
             "with a stated normal reaction to compare with; left for a later change")},
    {"case": "a variable spring hanger", "field": "User Hanger: Spring Rate, Hanger Load",
     "page": _page("user_hanger", "User Hanger"),
     "why": ("the page states the fields but not the unit the window takes them in for an SI model (its examples are in "
             "lb/in and lb); the designed Hanger type selects from a manufacturer's catalogue, which is not invented "
             "data (https://docs.sstcae.com/tech_manual/hanger.htm)")},
    {"case": "a uniform lateral wind load", "field": "every wind field",
     "page": "none: no saved public page documents the wind load window",
     "why": "no field is stated, so wind_pressure_relation cannot be carried by an instruction"},
    {"case": "a second thermal set", "field": "Number of Thermal Loads, and T2 under Misc > Loads",
     "page": "https://docs.sstcae.com/users_manual/analysis.htm (\"Temperature\"), read 2026-11-12",
     "why": ("two changes, not one (the number of thermal loads and the second temperature), and the page does not state "
             "the unit the window takes T2 in")},
]


def _bar(model_id: str, *, weight: bool, thermal: bool, up_force: float | None = None):
    b = Builder(model_id, 0)
    b.material = DESIGNED_MATERIAL
    b.provenance = PROVENANCE
    a, mid, tip = b.node("A", (0, 0, 0)), b.node("M", (SPAN / 2, 0, 0)), b.node("B", (SPAN, 0, 0))
    pipe = b.run("P", [a, mid, tip], OD, WALL)[0]
    anchor = b.anchor("A", a)
    if up_force is not None:
        b.force("up", tip, "global_y", up_force)
    if weight:
        b.add_weight()
    if thermal:
        b.add_thermal(DELTA_T)
    return b, anchor, tip, pipe, mid


def _support(member_id: str, node: str, **fields: Any) -> dict[str, Any]:
    return {"id": member_id, "node": node, "restraints": [], "provenance": PROVENANCE, **fields}


def _one_way(member_id: str, node: str, dof: str, active_when: str) -> dict[str, Any]:
    return _support(member_id, node, family="nonlinear", nonlinear={
        "behavior": "one_way", "dof": dof, "initial_state": "active", "active_when": active_when})


def _limit_stop_fields(*, upper: str, lower: str, direction: tuple[str, str, str] | None, preset: str | None):
    unit_zero = ("none needed: the value is zero, the same in every length unit; the page does not state the window's "
                 "length unit, and no other value is entered")
    sign = ("the gap in front of the node in the direction of the vector is the upper limit, the gap to the rear the "
            "lower limit, measured from the undeflected position of the node; a zero limit makes a one-way restraint in "
            "that direction; a blank limit means the node moves freely on that side")
    fields = []
    if preset:
        fields.append({"field": "Vertical (preset button)", "value": "pressed",
                       "short": ("it enters Lower limit 0, Upper limit blank and Y comp 1: a one-way stop that keeps the "
                                 "node from moving down; zero needs no unit"),
                       "enters": "Lower limit 0.000, Upper limit blank (None), Y comp 1.000", "unit": unit_zero,
                       "sign_convention": sign + "; with Y vertical and Y comp 1 the zero lower limit stops the node "
                                                 "moving down", "page": LIMIT_STOP})
    else:
        for name, value in (("Upper limit", upper), ("Lower limit", lower)):
            where = "in front of the node along the direction" if name == "Upper limit" else "behind the node"
            fields.append({"field": name, "value": value,
                           "short": (f"the gap {where}; zero makes a one-way restraint, no unit needed" if value == "0"
                                     else f"no limit {where}, the node moves freely on that side"),
                           "unit": unit_zero if value == "0" else "none: left blank",
                           "sign_convention": sign, "page": LIMIT_STOP})
        for axis, value in zip("XYZ", direction):
            fields.append({"field": f"{axis} comp", "value": value, "short": f"global {axis} component, no unit",
                           "unit": "none: a component of the direction in global X, Y and Z",
                           "sign_convention": "the limit stop's direction vector in global components", "page": LIMIT_STOP})
    fields += [
        {"field": "Friction coefficient", "value": "blank", "short": "no friction", "unit": "none (a coefficient)",
         "sign_convention": "not applicable: left blank, no friction", "page": LIMIT_STOP},
        {"field": "Stiffness", "value": "Rigid (the default)", "short": "the page's default", "unit": "none: the Rigid token",
         "sign_convention": "not applicable", "page": LIMIT_STOP},
        {"field": "Connected to Node", "value": "blank", "short": "connected to the ground", "unit": "none",
         "sign_convention": "not applicable", "page": LIMIT_STOP},
    ]
    return fields


def _instruction(run_id: str, node_label: str, what: str, fields: list[dict[str, Any]], page: str,
                 lost_if_dropped: str) -> str:
    """The run's window instruction, in numbered steps; the files follow the pack: each run is its own case under its
    run id."""
    return numbered([
        (f"Import {run_id}.mbf: File > Import from the Main window, select the file and click Import.", (IMPORT,)),
        (f"In the Layout window, at node {node_label}, enter {what} with the fields of the next steps.", (page,)),
        *[(f"{item['field']}: {item['value']} ({item['short']}).", ()) for item in fields],
        ("Leave every other field and record as imported.", ()),
        *finish_steps(run_id, "hand", export_lost=lost_if_dropped),
    ])


def _case_n1(model_id):
    b, anchor, tip, pipe, _ = _bar(model_id, weight=False, thermal=True)
    _, e, _, alpha, _ = b.material
    area, _ = section_properties(OD, WALL)
    force = e * area * alpha * DELTA_T
    member = [_one_way("support:B-stop", tip, "UX", "negative_reaction")]
    third = [closed("reaction_force", "x", EXPANSION, force, "N", "E A (alpha dT L - g)/L with g = 0 (anchor on pipe)",
                    basis="statics of an anchored bar against a closed one-way stop; invented inputs", support=anchor),
             closed("reaction_force", "x", EXPANSION, -force, "N", "-E A alpha dT (the stop on the pipe)",
                    basis="statics of an anchored bar against a closed one-way stop; invented inputs",
                    support="support:B-stop"),
             closed("displacement", "x", EXPANSION, 0.0, "m", "0 (the closed stop holds the free end)", node=tip)]
    return b, member, third, {"support:B-stop": {EXPANSION: "closed"}}, {EXPANSION: [RIGID]}, dict(
        label="stop-axial-closed", what="a Limit Stop (type L in the Data column, or Limit Stop in the Data Types dialog)",
        fields=_limit_stop_fields(upper="0", lower="blank", direction=("1", "0", "0"), preset=None),
        page=LIMIT_STOP, fields_entered=4, one_way=True,
        member_text=("a one-way stop along +X at the free end, no clearance in front: the bar's thermal growth closes it at "
                     "once"),
        description=("an anchored straight bar heated by dT against a one-way stop with no clearance at its free end along +X: "
                     "closed, carrying E A alpha dT"))


def _case_n4(model_id):
    b, anchor, tip, pipe, _ = _bar(model_id, weight=False, thermal=True)
    _, e, _, alpha, _ = b.material
    member = [_one_way("support:B-stop", tip, "UX", "positive_reaction")]
    third = [closed("reaction_force", "x", EXPANSION, 0.0, "N", "0 (the stop is open)", basis="statics; invented inputs",
                    support=anchor),
             closed("displacement", "x", EXPANSION, alpha * DELTA_T * SPAN, "m", "alpha dT L (free growth, stop open)",
                    node=tip)]
    return b, member, third, {"support:B-stop": {EXPANSION: "open"}}, {EXPANSION: [RIGID]}, dict(
        label="stop-axial-open", what="a Limit Stop (type L in the Data column, or Limit Stop in the Data Types dialog)",
        fields=_limit_stop_fields(upper="blank", lower="0", direction=("1", "0", "0"), preset=None),
        page=LIMIT_STOP, fields_entered=4, one_way=True,
        member_text="a one-way stop along +X at the free end, no clearance behind: the bar grows away from it",
        description="the bar of cpb-n-01 with the stop behind the free end: open, carrying nothing")


def _case_rest(model_id, *, lift: bool):
    b, anchor, tip, pipe, mid = _bar(model_id, weight=True, thermal=False)
    w = b.weight_per_length(pipe)  # negative: downward
    _, e, _, _, _ = b.material
    _, inertia = section_properties(OD, WALL)
    ei = e * inertia
    member = [_one_way("support:B-rest", tip, "UY", "positive_reaction")]
    basis = "Euler-Bernoulli cantilever and propped cantilever, ideal supports; invented inputs"
    if lift:
        up = float(f"{-2 * w * SPAN:.4g}")  # an upward tip force of twice the span's weight, well above 3|w|L/8
        b, anchor, tip, pipe, mid = _bar(model_id, weight=True, thermal=False, up_force=up)
        third = [closed("reaction_force", "y", SUSTAINED, 0.0, "N", "0 (the one-way prop lifts off)", basis=basis,
                        support="support:B-rest"),
                 closed("displacement", "y", SUSTAINED, up * SPAN ** 3 / (3 * ei) + w * SPAN ** 4 / (8 * ei), "m",
                        "P L^3/(3EI) + w L^4/(8EI) (free cantilever)", basis=basis, node=tip),
                 closed("reaction_force", "y", SUSTAINED, -(up + w * SPAN), "N", "-(P + w L)", basis=basis,
                        support=anchor),
                 closed("reaction_moment", "z", SUSTAINED, -(up * SPAN + w * SPAN ** 2 / 2), "N*m",
                        "-(P L + w L^2/2)", basis=basis, support=anchor)]
        status = "open"
        extra = {"up_force": up}
        text = dict(label="rest-lift-off", description=(
            "a cantilever under weight and an upward tip force of twice the span's weight, on a one-way vertical support "
            "at the tip: the force exceeds the 3|w|L/8 the closed support would carry, so it lifts off"),
            member_text="a one-way vertical (resting) support at the tip")
    else:
        third = [closed("reaction_force", "y", SUSTAINED, -3 * w * SPAN / 8, "N", "3|w|L/8 (propped cantilever)",
                        basis=basis, support="support:B-rest"),
                 closed("displacement", "y", SUSTAINED, 0.0, "m", "0 (the closed prop)", basis=basis, node=tip),
                 closed("reaction_force", "y", SUSTAINED, -5 * w * SPAN / 8, "N", "5|w|L/8", basis=basis,
                        support=anchor),
                 closed("reaction_moment", "z", SUSTAINED, -w * SPAN ** 2 / 8, "N*m", "|w| L^2/8", basis=basis,
                        support=anchor)]
        status = "closed"
        extra = {}
        text = dict(label="rest-closed", description=(
            "a cantilever under weight on a one-way vertical support at the tip: closed, a propped cantilever"),
            member_text="a one-way vertical (resting) support at the tip")
    return b, member, third, {"support:B-rest": {SUSTAINED: status}}, {SUSTAINED: [SHEAR, RIGID]}, dict(
        what="a Limit Stop (type L in the Data column, or Limit Stop in the Data Types dialog)",
        fields=_limit_stop_fields(upper="", lower="", direction=None, preset="Vertical"), page=LIMIT_STOP,
        fields_entered=2, one_way=True, extra=extra, **text)


def _case_h1(model_id):
    b, anchor, tip, pipe, mid = _bar(model_id, weight=True, thermal=False)
    w = b.weight_per_length(pipe)
    load = -3 * w * SPAN / 8
    # One acting translational DOF, vertical (UY, upward positive), and a positive stated load: the two conditions on
    # which the product applies a constant-effort support's load (core/product_physics/src/lib.rs,
    # classify_constant_effort_consumption).
    member = [_support("support:B-constant", tip, family="constant_effort_support", restraints=["UY"], hanger={
        "hanger_type": "constant_effort_support", "constant_load": bm.q(load, "N"), "travel_range": bm.q(0.05, "m"),
        "manufacturer_reference": "invented benchmark constant-effort support (no catalogue)",
        "source_reference": "invented benchmark value: 3|w|L/8, the vertically restrained tip's reaction"})]
    basis = "statics of a cantilever under weight and a constant upward tip force; invented inputs"
    third = [closed("displacement", "y", SUSTAINED, 0.0, "m",
                    "0 (the constant force equals the restrained tip's reaction 3|w|L/8)", basis=basis, node=tip),
             closed("reaction_force", "y", SUSTAINED, -5 * w * SPAN / 8, "N", "5|w|L/8", basis=basis, support=anchor),
             closed("reaction_moment", "z", SUSTAINED, -w * SPAN ** 2 / 8, "N*m", "|w| L^2/8", basis=basis,
                    support=anchor)]
    fields = [
        {"field": "Number of Hangers", "value": "1",
         "short": "a count, no unit; the support pushes up with a constant force the program calculates",
         "unit": "none: a count of hangers in parallel",
         "sign_convention": "not applicable; the support exerts a constant vertical supporting force (upward)",
         "page": CONSTANT_SUPPORT},
        {"field": "Tag", "value": "blank", "short": "no tag", "unit": "none", "sign_convention": "not applicable",
         "page": CONSTANT_SUPPORT},
        {"field": "Connected to Node", "value": "blank", "short": "connected to the ground", "unit": "none",
         "sign_convention": "not applicable", "page": CONSTANT_SUPPORT},
    ]
    return b, member, third, {}, {SUSTAINED: [SHEAR, RIGID, CONSTANT_LOAD]}, dict(
        label="constant-support",
        what="a Constant Support (type cons in the Data column, or Constant Support in the Data Types dialog)",
        fields=fields, page=CONSTANT_SUPPORT, fields_entered=2, one_way=False,
        member_text=("a constant support at the tip acting upward on the vertical DOF; SWBPIPE's member states the "
                     "textbook load 3|w|L/8; the target program's window calculates the support's load itself (the "
                     "page)"),
        description=("a cantilever under weight with a constant support at the tip. The support's load is an input NOT "
                     "held equal, named constant_support_load: SWBPIPE states it (3|w|L/8), the target program's window "
                     "calculates it by its own procedure. SWBPIPE's stated load is never adjusted to what comes back"),
        load_note="SWBPIPE states the constant load; the target program calculates it (the page)")


# Ranked, most valuable first: what each would evidence for the benchmark against the owner's time at the window.
CASES = (
    ("N", 1, "cpb-n-01", _case_n1),
    ("N", 2, "cpb-n-02", lambda model_id: _case_rest(model_id, lift=True)),
    ("N", 3, "cpb-n-03", lambda model_id: _case_rest(model_id, lift=False)),
    ("N", 4, "cpb-n-04", _case_n4),
    ("H", 5, "cpb-h-01", _case_h1),  # last: its support's load is an input not held equal
)
FAMILY_TEXT = {"N": "nonlinear supports entered by hand at the window (one-way stops and supports, no clearance)",
               "H": "hangers entered by hand at the window (a constant support)"}


def hand_entry_model(document: dict[str, Any]) -> dict[str, Any]:
    """The SWBPIPE model of a hand-entry case: the base with the member's support records appended (the stated rule)."""
    model = copy.deepcopy(document["model"])
    model["supports"] = [*model["supports"], *copy.deepcopy(document["hand_entry"]["member"])]
    return model


def hand_entry_document(document: dict[str, Any]) -> dict[str, Any]:
    """The run document of a hand-entry case: the model with the member, its third values as closed forms and its
    inexact inputs; deterministic, nothing written to the tracked set."""
    run = copy.deepcopy(document)
    hand = run.pop("hand_entry")
    run["model"] = hand_entry_model(document)
    run["closed_form"] = hand["third_values"]
    run["run_id"] = hand["run"]["run_id"]
    run["capture_held"] = True
    return run


def documents(seed: int, common: dict[str, Any], static_classes) -> list[dict[str, Any]]:
    out = []
    for family, rank, model_id, build in CASES:
        b, member, third, status, inexact, spec = build(model_id)
        base = b.model()
        cases = sorted({case["id"] for case in base["load_cases"]} | {OPERATING})
        inexact_all = {case: inexact.get(case, []) for case in cases if case != OPERATING}
        inexact_all[OPERATING] = sorted({item for items in inexact_all.values() for item in items})
        nonlinear = family == "N"
        classes = {case: ("nonlinear" if nonlinear else static_classes(inexact_all, [case])[case]) for case in cases}
        writer = bm.render_caepipe_mbf_si(base, title=model_id, reference_temperature_c=bm.REFERENCE_TEMPERATURE_C,
                                          piping_code=bm.PIPING_CODE)
        node_label = str(writer.to_dict()["node_map"][member[0]["node"]])
        run_id = f"{model_id}--{spec['label']}"
        instruction = _instruction(run_id, node_label, spec["what"], spec["fields"], spec["page"],
                                   EXPORT_LOST_HAND[family])
        expected_status = {support: dict(by_case) for support, by_case in status.items()}
        if OPERATING in cases:
            for by_case in expected_status.values():
                [state] = set(by_case.values())
                by_case[OPERATING] = state
        out.append({
            "document_kind": bm.MODEL_KIND, "model_id": model_id, "family": family,
            "family_description": FAMILY_TEXT[family], **common, "sampling": "designed", "hand_entry_rank": rank,
            "parameters": {"od": OD, "wall": WALL, "span": SPAN, "material": DESIGNED_MATERIAL[0],
                           **({"delta_t": DELTA_T} if any(c["id"] == EXPANSION for c in base["load_cases"]) else {}),
                           **spec.get("extra", {})},
            "series": [], "description": spec["description"], "inexact_inputs": inexact_all,
            "screening_classes": {"static": classes},
            "closed_form": [],
            "hand_entry": {
                "member": member, "member_text": spec["member_text"],
                "base_rule": ("model is the WRITTEN BASE: the case's SWBPIPE model with the member's support records "
                              "left out; hand_entry_model appends them to model.supports and changes nothing else"),
                "third_values": third, "expected_status": expected_status,
                "held": {case: (HELD_UNTIL_SERIES if CONSTANT_LOAD in inexact_all[OPERATING] else HELD)
                         for case in cases},
                **({"one_way_case": ONE_WAY} if spec["one_way"] else {}),
                "run": {
                    "series": "hand-entry", "kind": "hand_entry", "label": spec["label"], "run_id": run_id,
                    "window_instruction": {"text": instruction,
                                           "sha256": hashlib.sha256(instruction.encode("utf-8")).hexdigest()},
                    "fields": spec["fields"], "fields_entered_estimate": spec["fields_entered"],
                    "expected_saved_file": f"{run_id}.mod", "expected_print_file": f"{run_id}.print.csv",
                    "saved_file_read": "nothing of SWBPIPE's reads the saved file",
                    "export": export_declaration(run_id, EXPORT_LOST_HAND[family]),
                    "pages": sorted({item["page"] for item in spec["fields"]} | {IMPORT, ANALYZE_PAGE, PRINT_PAGE,
                                                                               SAVE_PAGE, FILE_MENU_PAGE, EXPORT_PAGE}),
                },
            },
            "provenance": {"origin": "invented", "license": "project-original",
                           "redistribution_status": "public_permissive", "produced_by_caepipe": False,
                           "contributor_note": PROVENANCE},
            "model": base,
        })
    return out
