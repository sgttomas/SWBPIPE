import sys
P=sys.argv[1]; tail=open(sys.argv[2]).read()
m=P+"/src/retained_memory.rs"; s=open(m).read()
reps=[
("""enum RegisteredProfile {}
#[derive(Clone, Copy)]
pub(super) struct CapturePermit {
    _profile: &'static RegisteredProfile,
}""","""// DISPOSABLE ARCHIVE ONLY (decision 7): an inhabited stub profile, selected by a
// test-thread flag, to run the permitted dispatch end to end. Never maintained code.
#[derive(Clone, Copy)]
enum RegisteredProfile { Stub }
static STUB: RegisteredProfile = RegisteredProfile::Stub;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StubMode { Off, Permit, RefuseLate, RefuseComplete, NoStack }
thread_local! { static STUB_MODE: std::cell::Cell<StubMode> = const { std::cell::Cell::new(StubMode::Off) }; }
pub(crate) fn set_stub(mode: StubMode) { STUB_MODE.with(|m| m.set(mode)); }
fn stub_mode() -> StubMode { STUB_MODE.with(|m| m.get()) }
#[derive(Clone, Copy)]
pub(super) struct CapturePermit {
    _profile: &'static RegisteredProfile,
    mode: StubMode,
}"""),
("""    pub(super) fn reserved_stack_bytes(&self) -> usize {
        match *self._profile {}""","""    pub(super) fn reserved_stack_bytes(&self) -> usize {
        match (*self._profile, self.mode) { (RegisteredProfile::Stub, StubMode::NoStack) => 1usize << 62, (RegisteredProfile::Stub, _) => 64 << 20 }"""),
("""    pub(super) fn check_late(&self, _facts: &LateFacts<'_>) -> Result<(), PhaseRefusal> {
        match *self._profile {}""","""    pub(super) fn check_late(&self, _facts: &LateFacts<'_>) -> Result<(), PhaseRefusal> {
        match (*self._profile, self.mode) { (RegisteredProfile::Stub, StubMode::RefuseLate) => Err(PhaseRefusal { gate: PhaseGate::Late }), _ => Ok(()) }"""),
("""    pub(super) fn check_complete(&self, _facts: &CompleteFacts<'_>) -> Result<(), PhaseRefusal> {
        match *self._profile {}""","""    pub(super) fn check_complete(&self, _facts: &CompleteFacts<'_>) -> Result<(), PhaseRefusal> {
        match (*self._profile, self.mode) { (RegisteredProfile::Stub, StubMode::RefuseComplete) => Err(PhaseRefusal { gate: PhaseGate::Complete }), _ => Ok(()) }"""),
("""    // All counts and missing premises survive the refusal, including incomplete
    // traversal and overflow. A measured capacity is never promoted to a permit.
    Err(report)""","""    match stub_mode() {
        StubMode::Off => Err(report),
        mode => Ok(CapturePermit { _profile: &STUB, mode }),
    }"""),
("""        Ok(permit) => match *permit._profile {},""","""        Ok(permit) => match *permit._profile { RegisteredProfile::Stub => unreachable!("assess with a stub permit") },"""),
]
for a,b in reps:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
open(m,"w").write(s)
g=P+"/src/s11g_tests.rs"; gs=open(g).read(); old="\nfn c1_request(force: f64, moment: f64) -> Value {"
assert gs.count(old)==1; open(g,"w").write(gs.replace(old,"\npub(super) fn c1_request(force: f64, moment: f64) -> Value {"))
t=P+"/src/retained_facade_tests.rs"
open(t,"a").write(tail)
print("stub patched")
