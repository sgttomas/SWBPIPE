from pathlib import Path
k=Path('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained')
p=k/'adaptive.rs';s=p.read_text().replace('is_zero()??','is_zero()?').replace('!allowance.is_zero()','!allowance.is_zero()?')
s=s.replace('    PublicationSpent {\n        total,','    let total = finish_work(&mut result, meter.total(), &StageWork::default());\n    PublicationSpent {\n        total,')
# certify uses an immediate closure rather than run()
a=s.index('fn certify_publication(');b=s.index('\n}',a);chunk=s[a:b].replace('let result = (||','let mut result = (||');s=s[:a]+chunk+s[b:]
s=s.replace('pub enum AttemptStop {','pub enum AttemptStop {\n    CountRange(&\'static str),').replace('pub enum UnresolvedReason {','pub enum UnresolvedReason {\n    CountRange(&\'static str),').replace('match stop {\n        AttemptStop::WorkAccounting','match stop {\n        AttemptStop::CountRange(field) => Ok(UnresolvedReason::CountRange(field)),\n        AttemptStop::WorkAccounting')
s=s.replace('64 * screen.operations','screen.operations.checked_mul(64).ok_or(AttemptStop::CountRange("pivot multiplier"))?').replace('64 * m,','m.checked_mul(64).ok_or(AttemptStop::CountRange("residual multiplier"))?,').replace('let m = 2 * count + 2;','let m = count.checked_mul(2).and_then(|v| v.checked_add(2)).ok_or(AttemptStop::CountRange("residual operations"))?;')
s=s.replace('self.offered.checked_add(1).ok_or(AttemptStop::Structure)?','self.offered.checked_add(1).ok_or(AttemptStop::CountRange("tracker sequence"))?')
# A truthful distinct refusal for the one preparation method without a source error channel.
s=s.replace('impl CasePrep {','''pub(crate) enum CombinationPreparationError { Ledger(LedgerRefusal), CountRange(&'static str) }
impl From<LedgerRefusal> for CombinationPreparationError { fn from(e: LedgerRefusal) -> Self { Self::Ledger(e) } }

impl CasePrep {''')
s=s.replace('pub(crate) fn combination(operands: &[(f64, &CasePrep)]) -> Result<Self, LedgerRefusal> {','''pub(crate) fn combination(operands: &[(f64, &CasePrep)]) -> Result<Self, CombinationPreparationError> {
        let fail = || CombinationPreparationError::CountRange("combination encoding");
        u32::try_from(operands.len()).map_err(|_| fail())?;
        let mut bytes = 10usize;
        let mut loads = 0usize;
        for (_, prep) in operands {
            u32::try_from(prep.identity.len()).map_err(|_| fail())?;
            bytes = bytes.checked_add(12).and_then(|n| n.checked_add(prep.identity.len())).ok_or_else(fail)?;
            loads = loads.checked_add(prep.source.loads().len()).ok_or_else(fail)?;
        }
        std::alloc::Layout::array::<u8>(bytes).map_err(|_| fail())?;
        std::alloc::Layout::array::<(usize, super::super::super::exact_sum::ExactAccumulator)>(loads).map_err(|_| fail())?;
        let first = operands.first().ok_or_else(fail)?.1;
        let pairs = operands.len().checked_mul(first.prescribed.len()).ok_or_else(fail)?;
        std::alloc::Layout::array::<(f64, f64)>(pairs).map_err(|_| fail())?;''')
s=s.replace('super::super::super::exact_sum::ExactAccumulator','crate::exact_sum::ExactAccumulator')
s=s.replace('Self::with(first.clone(), ledger, prescribed, factors, identity)','Self::with(first.clone(), ledger, prescribed, factors, identity).map_err(CombinationPreparationError::from)')
p.write_text(s)
p=k/'wide_sum.rs';s=p.read_text().replace('        for magnitude in [&self.positive, &self.negative] {\n            if highest_bit(magnitude).is_some_and(|h| h.checked_add(shift).is_none_or(|v| v >= SUM_LIMBS * 64)) { return Err(self.inconsistent(false)); }\n        }','        let outside = [&self.positive, &self.negative].into_iter().any(|magnitude| highest_bit(magnitude).is_some_and(|h| h.checked_add(shift).is_none_or(|v| v >= SUM_LIMBS * 64)));\n        if outside { return Err(self.inconsistent(false)); }')
s=s.replace('        let (Some(lo), Some(hi)) =','        if t.len().checked_mul(64).is_none() { return Err(SumRefusal::Span); }\n        let (Some(lo), Some(hi)) =')
# exposed only inside the retained module; borrowed donors are checked even before zero branches
s=s.replace('    fn ensure_valid(&self)', '    pub(crate) fn ensure_valid(&self)')
p.write_text(s)
p=k/'adaptive.rs';s=p.read_text();s=s.replace('    let mut n = num.clone();','    num.ensure_valid()?;\n    den.ensure_valid()?;\n    let mut n = num.clone();');p.write_text(s)
p=k/'factor.rs';s=p.read_text().replace('64 * m,','m.checked_mul(64).ok_or(AttemptStop::CountRange("pivot multiplier"))?,').replace('let operations = 2 * (i - first[i]) as u64 + 2;','let operations = u64::try_from(i - first[i]).ok().and_then(|n| n.checked_mul(2)).and_then(|n| n.checked_add(2)).ok_or(AttemptStop::CountRange("factor operations"))?;').replace('lift_count(3 * n)?','lift_count(n.checked_mul(3).ok_or(AttemptStop::CountRange("condition multiplier"))?)?');p.write_text(s)
p=k/'bound.rs';s=p.read_text().replace('while r * r > n {','while u128::from(r) * u128::from(r) > u128::from(n) {').replace('while r * r < n {','while u128::from(r) * u128::from(r) < u128::from(n) {');p.write_text(s)
p=k/'combine.rs';s=p.read_text().replace('RetainedSolve, UnresolvedReason,','RetainedSolve, UnresolvedReason, CombinationPreparationError,').replace('pub enum CombinationReason {','pub enum CombinationReason {\n    CountRange(&\'static str),').replace('Err(e) => return withheld(CombinationReason::LedgerUnavailable(e)),','Err(CombinationPreparationError::Ledger(e)) => return withheld(CombinationReason::LedgerUnavailable(e)),\n            Err(CombinationPreparationError::CountRange(field)) => return withheld(CombinationReason::CountRange(field)),');p.write_text(s)
p=k/'source.rs';s=p.read_text().replace('pub enum SourceError {','pub enum SourceError {\n    CountRange(&\'static str),')
# raw lengths checked before test-only clones, sorting, construction or index casts
needle='''        // V-K seeded fault VK-F10'''
checks='''        source_counts(&nodes, &members, &springs, &directional_springs, &constraints, &loads, &stations, &supports)?;
'''
s=s.replace(needle,checks+needle,1)
# checked free/profile range after native semantic validation and before graph creation
s=s.replace('        // Bodies: the connected components','''        let free = node_count * DOF_PER_NODE - constraints.len();
        checked_profile_count(free)?;
        // Bodies: the connected components''')
a=s.index('impl PrimitiveSource {')
s=s[:a]+'''// Scalar representation checks only; these do not supply a memory allowance.
fn checked_profile_count(free: usize) -> Result<(), SourceError> {
    let product = free.checked_add(1).and_then(|n| free.checked_mul(n)).ok_or(SourceError::CountRange("free profile"))?;
    if free > u32::MAX as usize { return Err(SourceError::CountRange("free block sentinel")); }
    std::alloc::Layout::array::<usize>(product / 2).map_err(|_| SourceError::CountRange("profile layout"))?;
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn source_counts(nodes: &[[f64; 3]], members: &[StraightMember], springs: &[Spring], directional: &[DirectionalSpring], constraints: &[Constraint], loads: &[NodalLoad], stations: &[Station], supports: &[SupportGroup]) -> Result<(), SourceError> {
    let fail = || SourceError::CountRange("source representation");
    for len in [nodes.len(), members.len(), springs.len(), directional.len(), constraints.len(), loads.len(), stations.len(), supports.len()] { u32::try_from(len).map_err(|_| fail())?; }
    let n = nodes.len().checked_mul(DOF_PER_NODE).ok_or_else(fail)?;
    std::alloc::Layout::array::<Option<f64>>(n).map_err(|_| fail())?;
    n.checked_add(1).ok_or_else(fail)?;
    let mut bytes = 38usize;
    for (len, stride) in [(nodes.len(),24usize),(members.len(),84),(springs.len(),17),(directional.len(),41),(constraints.len(),13),(loads.len(),17),(stations.len(),16),(supports.len(),22)] {
        bytes = len.checked_mul(stride).and_then(|v| bytes.checked_add(v)).ok_or_else(fail)?;
    }
    for load in loads { u32::try_from(load.source_id.len()).map_err(|_| fail())?; bytes = bytes.checked_add(load.source_id.len()).ok_or_else(fail)?; }
    for group in supports { for children in [&group.springs, &group.directional_springs] {
        u32::try_from(children.len()).map_err(|_| fail())?;
        bytes = children.len().checked_mul(4).and_then(|v| bytes.checked_add(v)).ok_or_else(fail)?;
    } }
    std::alloc::Layout::array::<u8>(bytes).map_err(|_| fail())?;
    let mut q = 0usize;
    for (len, stride) in [(nodes.len(),7usize),(members.len(),12),(stations.len(),6),(springs.len(),1),(directional.len(),3),(constraints.len(),1),(supports.len(),2)] {
        q = len.checked_mul(stride).and_then(|v| q.checked_add(v)).ok_or_else(fail)?;
    }
    u32::try_from(q).map_err(|_| fail())?;
    std::alloc::Layout::array::<super::recover::QuantityMeta>(q).map_err(|_| fail())?;
    // Pattern, tags, prefix sums and transpose products before their construction.
    let p = members.len().checked_mul(144).and_then(|v| directional.len().checked_mul(9).and_then(|d| v.checked_add(d))).and_then(|v| v.checked_add(springs.len())).ok_or_else(fail)?;
    let z = n.checked_mul(n).ok_or_else(fail)?.min(p);
    for count in [p, z] { count.checked_mul(2).ok_or_else(fail)?; let prefix = count.checked_add(1).ok_or_else(fail)?; std::alloc::Layout::array::<usize>(prefix).map_err(|_| fail())?; }
    std::alloc::Layout::array::<(usize, usize)>(p).map_err(|_| fail())?;
    Ok(())
}

'''+s[a:]
p.write_text(s)
