
#[cfg(test)]
impl ProductCapture {
    pub(super) fn rv62_source_check(&mut self, case: &PreviewLoadCase) -> Result<(), CaptureError> { self.basis_source_case(case) }
    pub(super) fn rv62_record_check(&self, row: &ResultItem) -> Result<(), CaptureError> { self.validate_modulus_record(row) }
}
