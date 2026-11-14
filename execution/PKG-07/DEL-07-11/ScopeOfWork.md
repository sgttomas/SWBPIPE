---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-11
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-078]
package_objective_refs: [OBJ-006, OBJ-010, OBJ-015]
---

# Scope of Work — DEL-07-11 Desktop workspace shell and host integration

## Purpose and Objective Traceability

This deliverable is the frame the desktop's panels live in and the boundary between them and the native host. It mounts the panels, carries the session they share (the working model, the selection, the pending changes, the history and the current result), routes menus, commands, jobs and file operations to the service that owns each, and keeps what is transient apart from what is stored. It adds no engineering meaning and no policy of its own: a panel's content is the panel's, a service's rule is the service's. The result it contributes is one coherent workspace in which nothing reaches the model, the store or the file system except by an owned route, the user's work is not lost or replaced without their say, and private data stays where the user put it.

It serves SOW-078 under OBJ-006, OBJ-010 and OBJ-015 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-11), and PRD §14.1, §5.2, §5.3, §20.3, §20.4 and FR-GUI-005. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It supports criterion 9, whose desktop measurement runs in the scripted-run build defined here.

- **OUT-001** — The shell's routing: navigation, panel mounting, jobs and explicit file operations dispatched to their owning services.
- **OUT-002** — The shared session: selection, pending changes and history, kept apart from stored state.
- **OUT-003** — The native-host boundary: host configuration, private-data boundaries, cancellation, interruption and recovery.
- **OUT-004** — Common focus, keyboard and accessibility behaviour across panels.

## Deliverable Definition — Ontology

**Terms the shell implements.**

- *Shell.* The window, menus, toolbar, stage navigation, status bar and panel layout of the desktop, and the code that mounts each panel.
- *Owning service.* The service that validates and performs a request: the operation engine for a change to the model, the project store for persistence, the solve job for a solve, an exporter for an export. The shell dispatches to it and performs none of its work.
- *Session.* What the workspace holds while it is open: the working model, the selection, the pending changes, the history, the view state and the handle of the current result. *Transient* state is session state that is never stored: selection, view state, drafts and job progress. *Durable* state is the stored project and its run records.
- *Working model.* The model document the session is editing. A *replacing command* is any command that puts another document in its place: new, open, reopen a saved state, import, or close.
- *History.* The session's checkpoints. One successfully applied batch is one checkpoint. Undo and redo restore checkpoints. A result computed on a model other than the working model is marked as not current.
- *Explicit file operation.* A read or write of a file the user chose in a native dialog. Nothing else reads or writes outside the application's own storage.
- *Model document file.* A file holding one model document. Opening one adopts the document exactly as read.
- *Suggested file name.* The name a save dialog proposes. The user may type any other.
- *Scripted-run build.* A build of the desktop made to be driven by a script, in which the file the chooser returns and the native menu commands are substituted. Its substitution and instrumentation are compiled only into that build. In it, every Keychain item operation the application owns (loading, creating or deleting a saved provider key, and the default-keychain lookup they use) is refused, before any call to the Security framework, by a denial named as this build's own, so a scripted run reaches no Keychain item and shows nothing of how the product uses the Keychain. The native smoke runner builds it with a web view that selects a non-persistent data store, so a run by that runner does not show that a product build keeps web-view storage from one launch to the next; what the project store keeps is read from the store itself and is not affected.

**Boundary.** The shell hosts and coordinates. The following belong elsewhere:

- each panel's content and commands: the deliverable that owns the panel;
- the review, Apply and ledger interaction: DEL-07-08. Application of operations: DEL-16-06;
- the solve job, its progress and its result custody: DEL-07-07;
- the stored project, its revisions, autosave and recovery: DEL-02-05. REQ-006's save in place relies on the store refusing by name a save over a revision the session did not open or write and the user did not confirm (DEL-02-05/REQ-036). The model document's form, version and validation: DEL-02-01. Run records: DEL-14-02;
- saving the working model as a model document file: DEL-15-03. The export formats and their review panels: PKG-17 and DEL-07-12;
- where private data may be stored and what may leave the machine: DEL-12-01 and DEL-12-02;
- the build and packaging of the desktop: DEL-10-04. That no product build contains the scripted-run build's features is this deliverable's outcome (REQ-010, AC-008); that none contains the native-smoke kit is DEL-04-05/AC-002;
- the accessibility target and presentation conventions: DEL-07-06;
- the command, query and job rules and the GUI state rule: DEL-00-03 and DEL-00-05.

**Not decided.** What the native host's security configuration is: the webview's content policy and the set of native commands the webview may call. No scope states it.

**Windows.** Windows is not a release platform for v1.0. A Windows implementation of the shell's native boundary keeps the native command names, owning services, admission rules and stale-result guards the macOS desktop has, and adds native code only at the operating-system operations. Its dispatch of menus, native dialogs, accelerators and file operations, and its security configuration, are examined in the launched Windows application; a run in a browser alone does not show them, and a Windows check of another native boundary does not meet them.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Shell navigation, panel mounting, jobs and explicit file operations are dispatched to their owning services, and the identity of what was sent and of what came back is preserved. The shell cannot perform a request its owning service would refuse.
- **REQ-002** — The shell coordinates one selection and one session history for every panel, and keeps transient session state out of the stored project and the run records. The session holds the working model with its hash and revision, registers a queued batch unchanged, takes an applied document as the working model, and keeps the document it replaced as one checkpoint. A queued batch may carry a precondition for what the batch rests on outside the model: the session keeps it beside the queue entry and outside the batch, checks it when the person applies the batch, after its own checks and before the host applies, and on a refusal applies nothing and shows the reason. A precondition that fails or does not answer within 10 seconds refuses. A precondition can only refuse. A batch whose contents require a precondition is not registered without one.
- **REQ-003** — The native host runs under its stated security configuration; private data stays in locations the user controls; a cancelled job, an interrupted session and a denied path each leave the session recoverable and say what happened.
- **REQ-004** — Focus, keyboard and accessibility behaviour common to the panels conform to DEL-07-06. Each panel keeps its own commands and meaning.
- **REQ-005** — *Opening a model document.* File › Open Model Document… reads a file through the native host and adopts its document exactly as read: no default, migration or normalization is applied, and the model hash a solve of it reports is the SHA-256 of the document's RFC 8785 canonical form. A file that cannot be adopted is refused by name and the session is unchanged: unreadable, too large, not JSON, not a model document, another document kind, refused by the schema, or of a shape the desktop cannot hold, with the paths concerned. In a browser preview the command says it is available only in the native desktop.
- **REQ-006** — *No silent loss or replacement.* Every replacing command, and closing the window, with unsaved edits opens one modal prompt that offers to save and continue, to discard and continue, or to cancel. Cancel is the default, holds focus, and is what Escape does. A save in place proceeds without a prompt only where the stored project is still the revision this session last opened or saved; otherwise the user is asked before anything is replaced.
- **REQ-007** — *No implicit model.* The desktop opens no model the user did not choose. The bundled invented example opens only on request and is labelled as invented. A command that needs a model and has none is refused by name.
- **REQ-008** — *Response integrity.* A response that does not answer the request the session last made (superseded, failed, or carrying a model other than the one sent) is refused by name and leaves the session as it was.
- **REQ-009** — *Suggested file names.* Every suggested name follows one convention: a fixed word for each kind of export and, where the export covers a case, filter, run or state, a qualifier naming it, or its stable key where the name reduces to nothing, in lower-case ASCII letters and digits joined by single hyphens, with the file's extension. No suggested name contains a project name, a project or model identifier, or a path. A name already taken is resolved by the native dialog.
- **REQ-010** — *Scripted run.* The desktop can be driven through a scripted run in the scripted-run build. No product build contains the substitution or the instrumentation.

### Acceptance criteria

- **AC-001** — For every native command and mounted route, with a valid, a failing and a denied request: the request reaches its owning service, a refusal is shown by the service's name for it, and no route changes the model or the store without the owning service's validation.
- **AC-002** — After save and reopen, the model document and run identities equal those saved and no selection, view state or draft is in the stored bytes; one applied batch is one checkpoint that undo restores and redo reapplies; a superseded or mismatched response is refused by name with the session unchanged; a batch whose precondition refuses is not applied, the model and history are unchanged, and the batch stays queued with the refusal shown on its entry; a precondition never causes a batch to be applied that the session's own checks refuse, nor delays an apply past a change of the model's revision; and a batch whose contents require a precondition is refused registration without one.
- **AC-003** — In the native desktop: each menu item and host event reaches its command; a cancelled job, a session interrupted and reopened, a denied path and a path outside the user's chosen locations each end as REQ-003 states; and no private data is written outside those locations.
- **AC-004** — Focus passes between panels in the stated order, every shell command can be reached and run by keyboard, and the shell's own controls meet DEL-07-06's target.
- **AC-005** — For a file of each refused kind: the named refusal is given and the session is unchanged. For a valid file: the working model's canonical form equals the file's, and the model hash a solve reports equals the SHA-256 of that canonical form computed apart from the product.
- **AC-006** — With unsaved edits, every replacing command and window close opens the prompt with Cancel focused; Cancel and Escape leave the model, the store and the window as they were; and a save in place prompts exactly when the stored revision is not the one the session last opened or saved.
- **AC-007** — The suggested name of every export in the desktop follows the convention with its kind's word and extension, and none contains the open project's name, its identifier, the model's identifier or a path.
- **AC-008** — A scripted run completes in the scripted-run build through the substituted chooser and menus, and a product build contains none of the substitution or instrumentation.
- **AC-009** — At start no model is open until the user chooses one; the bundled example opens only from its command and is labelled as invented; and each command that needs a model, run with none open, is refused by its name and changes nothing.

## Production and Verification Method — Praxeology

**Method choices.** The session is one store that panels read and act on through the shell, so no panel holds a private copy of the model or the selection. Each native command is a thin entry that names its owning service. The work-loss guard is one function every replacing command passes through. The suggested-name rule is one function every save dialog calls.

**How each criterion is examined.**

- **VER-001** — Executable check (observes each native command, event and mounted route): sends a valid, a failing and a denied request to each and reads which service received it, the refusal shown, and the model and store afterwards.
- **VER-002** — Executable check (observes a session through save, reopen, undo and redo): compares the model and run identities and the stored bytes with those saved, counts checkpoints against applied batches, and delivers a superseded and a mismatched response and reads the session.
- **VER-003** — Executable check (observes the native desktop, not a browser preview): drives menus and host events, cancels a job, interrupts and reopens a session, and requests a denied path and a path outside the user's locations, and reads the session and the file system after each.
- **VER-004** — Executable check (observes focus, keyboard input and the accessibility tree of the shell): walks focus between panels, runs each shell command by keyboard, and compares the shell's controls with DEL-07-06's target.
- **VER-005** — Comparison with a named reference (RFC 8785 canonical form and SHA-256, computed apart from the product): opens a file of each refused kind and a valid file, reads the refusal or the adopted document, and compares the working model's canonical form and the solve's reported model hash with the reference.
- **VER-006** — Executable check (observes a session with unsaved edits): runs each replacing command and window close, reads the prompt, its focus and the effect of each choice, and saves in place against a matching and a changed stored revision.
- **VER-007** — Executable check (observes every save dialog's suggested name): collects the suggested name of every export for an open project whose name and identifiers are known, and tests each against the convention and for those strings.
- **VER-008** — Executable check (observes the scripted-run build and a product build): runs a script to completion in the first and scans the second for the substitution and instrumentation.
- **VER-009** — Executable check (observes the desktop from start): reads the open model at start, opens the bundled example and reads its label, and runs each command that needs a model with none open and reads the refusal and the session.

## Governing Values and Decisions — Axiology

- **AX-001** — The shell carries; it does not decide. Better is a request that reaches its owning service unchanged and a refusal shown in that service's words, never a shell that validates, repairs or completes a request itself.
- **AX-002** — The user's work is theirs to keep or discard. Better is a prompt that defaults to doing nothing, measured by no replacing command proceeding without a choice, never a silent overwrite, discard or reload.
- **AX-003** — What is stored is only what was meant to be stored. Better is a stored project free of session state and a file name free of project identity, measured by scanning the bytes and the names, never convenience bought by leaking either.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-078 OBJ-006 OBJ-015 | REQ-001 | AC-001 | VER-001 | Each owning service's own validation and refusal names are the reference; the shell writes none of them. |
| OUT-002 | SOW-078 OBJ-006 OBJ-015 | REQ-002 REQ-008 | AC-002 | VER-002 | DEL-00-05's GUI state rule sets which state is transient. The expected identities and bytes are those the check saved, so the comparison is a round trip against what the product was given, not against its own later output. |
| OUT-003 | SOW-078 OBJ-010 | REQ-003 | AC-003 | VER-003 | PRD §20.3 keeps private data in user-controlled paths and §20.4 keeps the product's work local. The file system and the host are read directly; evidence from a browser preview does not count for this criterion. |
| OUT-004 | SOW-078 OBJ-006 | REQ-004 | AC-004 | VER-004 | The accessibility target is DEL-07-06's, from WCAG 2.2 level AA. |
| OUT-001 | SOW-078 OBJ-006 OBJ-010 | REQ-005 | AC-005 | VER-005 | Outside source: RFC 8785 and SHA-256, computed apart from the product. The refusals of form are the document validator's (DEL-02-01). |
| OUT-002 | SOW-078 OBJ-006 OBJ-010 | REQ-006 | AC-006 | VER-006 | The requirement defines the expected prompt, focus and effect of each choice. The comparison is against the model, the stored project and the window as they were before the command, and the stored revision the check itself wrote. |
| OUT-001 | SOW-078 OBJ-010 | REQ-009 | AC-007 | VER-007 | The requirement states the convention; the project's name and identifiers are set by the check, so their absence is tested against known strings. |
| OUT-003 | SOW-078 OBJ-006 | REQ-010 | AC-008 | VER-008 | The requirement states what a product build must not contain; the build is scanned as built. |
| OUT-001 | SOW-078 OBJ-006 | REQ-007 | AC-009 | VER-009 | The requirement defines what must happen; the comparison is against the session before each command. |
