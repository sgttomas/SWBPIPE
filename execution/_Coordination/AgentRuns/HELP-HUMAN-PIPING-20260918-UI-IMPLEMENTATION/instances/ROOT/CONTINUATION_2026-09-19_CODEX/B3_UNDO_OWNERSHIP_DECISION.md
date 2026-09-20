# B3 engineering decision — focus-aware Undo/Redo shortcut ownership

ROOT disposition during the approved B3 implementation, 2026-09-19. This is an
engineering decision inside native/browser menu-keyboard equivalence scope,
not a new owner ruling. UX specification section 2.5 requires Undo/Redo
accelerators. The manager found the inherited omission and identified that a
custom AppKit accelerator can preempt the text editor before DOM focus guards
can run; its menu callback also cannot distinguish a click from a shortcut.

Implement CmdZ/ShiftCmdZ (platform command modifier as appropriate) through the
existing webview key handler in both runtimes. Respect defaultPrevented and
focused input, textarea and contenteditable guards. Invoke the same
edit.undo/edit.redo command sink and preserve history/busy/result-invalidation
gates. Leave custom AppKit Undo/Redo accelerators unassigned; native menu
pointer commands remain synced. Other native-owned shortcuts retain their
ownership. This is a named ownership exception, not a native keyboard gap.

Actual Tauri witness must prove: with model/workspace focus, one keypress
causes one model undo/redo; inside a text input, normal text undo remains and
model history does not change. A host failure to deliver the key returns to
ROOT with evidence; configuration/unit assertions alone do not prove this.
The change is reversible in key binding ownership without changing model or
persistence contracts. No protected geometry/performance check changes.
