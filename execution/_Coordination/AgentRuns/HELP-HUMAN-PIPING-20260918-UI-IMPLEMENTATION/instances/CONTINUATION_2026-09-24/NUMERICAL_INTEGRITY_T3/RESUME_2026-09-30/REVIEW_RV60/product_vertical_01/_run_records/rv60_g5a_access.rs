// Test-only RV60 inspection access, external overlay only. No behavior replacement.
#[cfg(test)]
impl ProductCapture {
    pub(super) fn rv60_check_g5a(&mut self, owner:&k::RetainedSolve, rows:&[k::ProductFinalRow<'_>])->Result<(),G5aFailure>{self.g5a(owner,rows)}
}
