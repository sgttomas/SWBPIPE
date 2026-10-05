#!/usr/bin/env python3
"""RV85's own disposable permit stub (decision 7: archive only, never maintained code).
Written independently of I61's stub. Process-global switches (not thread-locals), so
they reach the reserved-stack thread."""
import sys
PP = sys.argv[1]
def sub(path, old, new):
    t = open(path).read(); assert t.count(old) == 1, (path, old[:60], t.count(old)); open(path, 'w').write(t.replace(old, new))
mem = PP + '/src/retained_memory.rs'
sub(mem, 'enum RegisteredProfile {}\n',
 'pub(crate) struct RegisteredProfile;\n'
 'static RV85_PROFILE: RegisteredProfile = RegisteredProfile;\n'
 'pub(crate) mod rv85_stub {\n'
 '    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};\n'
 '    pub(crate) static ADMIT: AtomicBool = AtomicBool::new(false);\n'
 '    pub(crate) static ADMIT_HEADLESS: AtomicBool = AtomicBool::new(false);\n'
 '    pub(crate) static LATE_REFUSE: AtomicBool = AtomicBool::new(false);\n'
 '    pub(crate) static COMPLETE_REFUSE: AtomicBool = AtomicBool::new(false);\n'
 '    pub(crate) static STACK: AtomicUsize = AtomicUsize::new(64 << 20);\n'
 '    pub(crate) static LATE_CALLS: AtomicUsize = AtomicUsize::new(0);\n'
 '    pub(crate) static COMPLETE_CALLS: AtomicUsize = AtomicUsize::new(0);\n'
 '    pub(crate) fn get(b: &AtomicBool) -> bool { b.load(SeqCst) }\n'
 '}\n')
sub(mem, '    pub(super) fn reserved_stack_bytes(&self) -> usize {\n        match *self._profile {}\n    }',
         '    pub(super) fn reserved_stack_bytes(&self) -> usize {\n        let _ = self._profile; rv85_stub::STACK.load(std::sync::atomic::Ordering::SeqCst)\n    }')
sub(mem, '    pub(super) fn check_late(&self, _facts: &LateFacts<\'_>) -> Result<(), PhaseRefusal> {\n        match *self._profile {}\n    }',
         '    pub(super) fn check_late(&self, _facts: &LateFacts<\'_>) -> Result<(), PhaseRefusal> {\n        rv85_stub::LATE_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);\n        if rv85_stub::get(&rv85_stub::LATE_REFUSE) { Err(PhaseRefusal { gate: PhaseGate::Late }) } else { Ok(()) }\n    }')
sub(mem, '    pub(super) fn check_complete(&self, _facts: &CompleteFacts<\'_>) -> Result<(), PhaseRefusal> {\n        match *self._profile {}\n    }',
         '    pub(super) fn check_complete(&self, _facts: &CompleteFacts<\'_>) -> Result<(), PhaseRefusal> {\n        rv85_stub::COMPLETE_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);\n        if rv85_stub::get(&rv85_stub::COMPLETE_REFUSE) { Err(PhaseRefusal { gate: PhaseGate::Complete }) } else { Ok(()) }\n    }')
sub(mem, '        Ok((permit, _)) => match *permit._profile {},\n',
         '        Ok((_permit, _)) => unreachable!("RV85 stub: assess is not called with ADMIT on"),\n')
sub(mem, '    // traversal and overflow. A measured capacity is never promoted to a permit.\n    Err(report)\n',
         '    // traversal and overflow. A measured capacity is never promoted to a permit.\n'
         '    let admitted = rv85_stub::get(&rv85_stub::ADMIT) && (report.caller == RetainedCaller::Direct || rv85_stub::get(&rv85_stub::ADMIT_HEADLESS));\n'
         '    if admitted { Ok(CapturePermit { _profile: &RV85_PROFILE }) } else { Err(report) }\n')
lib = PP + '/src/lib.rs'
sub(lib, '#[cfg(test)]\nmod retained_facade_tests;\n', '#[cfg(test)]\nmod retained_facade_tests;\n#[cfg(test)]\nmod rv85_stub_tests;\n')
