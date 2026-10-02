
#[cfg(test)]
mod rv33_boundary_checks {
    use super::*;
    #[test]
    fn poisoned_alternatives_never_disappear() {
        let poison=C::new(u128::MAX)+1;
        for c in [poison, poison.max(C::new(1)), C::new(1).max(poison), poison.min(C::new(0)), max(&[C::new(1),poison]), C::new(0)-1, C::new(u128::MAX)*2,C::new(1)/0,capacity(1,C::new((1u128<<127)+1)),a(8,C::new(1u128<<127))] {
            assert_eq!(c.raw(),Err(EnvelopeError::ArithmeticOverflow));
        }
        assert_eq!(Pair::exact(C::new(10)).max(Pair::exact(poison)).finish(),Err(EnvelopeError::ArithmeticOverflow));
        assert_eq!(C::new((1u128<<63)-1).bytes(),Ok((1u128<<63)-1));
        assert_eq!(C::new(1u128<<63).bytes(),Err(EnvelopeError::ReferenceWidthExceeded));
    }
    #[test]
    fn minima_growth_old_sort_and_tree_transitions() {
        for (stride,k,cap,grow,prev) in [(1,0,0,0,0),(1,1,8,8,0),(1,8,8,8,0),(1,9,16,16,8),(40,1,4,160,0),(40,4,4,160,0),(40,5,8,320,160),(4304,1,1,4304,0),(4304,2,2,8608,4304)] {
            assert_eq!(capacity(stride,C::new(k)).raw(),Ok(cap));assert_eq!(g(stride,C::new(k)).raw(),Ok(grow));assert_eq!(old(stride,C::new(k)).raw(),Ok(prev));
        }
        assert_eq!(sort(8,C::new(1)).raw(),Ok(0));assert_eq!(sort(8,C::new(2)).raw(),Ok(384));assert_eq!(sort(8,C::new(49)).raw(),Ok(392));
        for (k,nodes) in [(0,0),(1,1),(5,1),(6,2),(10,2),(11,3)]{assert_eq!(tree(C::new(k),240).raw(),Ok(nodes*240));}
    }
    #[test]
    fn pair_composition_has_only_one_active_old_request() {
        let result=Pair::new(C::new(5),C::new(3)).add(Pair::new(C::new(7),C::new(10))).finish().unwrap();
        assert_eq!(result,MetricBytes{requested:12,moving:22});
        let mut schedule=ScheduleEnvelope::new();
        for _ in 0..96{schedule.put(KernelPhase::GroupGeometry,None,Pair::exact(C::new(1))).unwrap();}
        assert_eq!(schedule.put(KernelPhase::GroupGeometry,None,Pair::exact(C::new(2))),Err(EnvelopeError::InternalPhaseCapacity));assert_eq!(schedule.phases().len(),96);
    }
    fn input(nodes:u128,free:u128,loads:u128,bytes:u128,max_id:u128)->KernelInput{
        let constraints=6*nodes-free;
        KernelInput{nodes,members:0,axis_springs:0,directional_springs:0,constraints,load_terms:loads,stations:0,load_id_bytes:bytes,max_load_id_bytes:max_id,support_groups:0,nonzero_prescribed_terms:0,structure:Some(StructuralCounts{dofs:6*nodes,free_dofs:free,quantities:7*nodes+constraints,source_encoding_bytes:38+24*nodes+13*constraints+17*loads+bytes,pattern_entries:0,profile_entries:free})}
    }
    fn check(i:KernelInput,p:PopulationPolicy)->Result<KernelDescriptor,EnvelopeError>{KernelDescriptor::new(i,SourceConstruction::VrModelV1,p)}
    #[test]
    fn zero_and_small_free_populations_keep_finite_complete_schedules(){
        for free in [0,1,2,3,4,5,7,8,9,15,16,17,511,512,513] {
            let i=input(100,free,0,0,0);let exact=check(i,PopulationPolicy::Exact{bodies:1,free_blocks:if free==0{0}else{1}}).unwrap();let upper=check(i,PopulationPolicy::NodesAndFreeDofsUpper).unwrap();let p=ReferenceKernelProfile::source40129_rust1971_aarch64_v1();let ex=kernel_envelope(&exact,&p).unwrap();let up=kernel_envelope(&upper,&p).unwrap();
            for (small,large) in [(&ex.full,&up.full),(&ex.selected128,&up.selected128)] {
                assert_eq!(small.phases().len(),large.phases().len());
                for (s,l) in small.phases().iter().zip(large.phases()) {assert_eq!(s.id,l.id);assert!(l.bytes.requested>=s.bytes.requested);assert!(l.bytes.moving>=s.bytes.moving);assert!(s.bytes.moving>=s.bytes.requested);}
                assert!(small.returned.union<=small.solve.requested);assert!(large.returned.union<=large.solve.requested);
            }
            assert_eq!(ex.full.phases().len(),88);assert_eq!(ex.selected128.phases().len(),38);assert!(ex.selected128.returned.union>=ex.base+ex.shared[0]+ex.solved[0]);
        }
    }
    #[test]
    fn descriptor_rejections_preserve_width_and_population_contracts(){
        let upper=PopulationPolicy::NodesAndFreeDofsUpper;
        assert!(matches!(check(input(0,0,0,0,0),upper),Err(EnvelopeError::InvalidDescriptor("empty source"))));
        let mut i=input(1,0,0,0,0);i.constraints=7;assert!(matches!(check(i,upper),Err(EnvelopeError::InvalidDescriptor("constraints exceed DOFs"))));
        let mut i=input(1,0,0,0,0);i.structure.as_mut().unwrap().pattern_entries=1;assert!(matches!(check(i,upper),Err(EnvelopeError::InvalidDescriptor("pattern/profile population"))));
        let mut i=input(1,3,0,0,0);i.structure.as_mut().unwrap().profile_entries=2;assert!(matches!(check(i,upper),Err(EnvelopeError::InvalidDescriptor("pattern/profile population"))));
        for (f,b,bc) in [(0,1,1),(1,1,0),(1,1,2),(1,2,1)]{assert!(matches!(check(input(1,f,0,0,0),PopulationPolicy::Exact{bodies:b,free_blocks:bc}),Err(EnvelopeError::InvalidDescriptor("body/block counts"))));}
        for (l,bytes,max_id) in [(0,1,0),(0,0,1),(1,1,0),(2,2,2),(2,5,2)] {assert!(matches!(check(input(1,1,l,bytes,max_id),upper),Err(EnvelopeError::InvalidDescriptor("load ID byte counts"))));}
        assert!(check(input(1,1,2,3,2),upper).is_ok()); // UTF-8 byte length arithmetic, not character count.
        assert!(matches!(check(input(u128::from(u32::MAX)+1,0,0,0,0),upper),Err(EnvelopeError::ReferenceWidthExceeded)));
        assert!(matches!(check(input(1,1,1,u128::from(u32::MAX)+1,u128::from(u32::MAX)+1),upper),Err(EnvelopeError::ReferenceWidthExceeded)));
    }
}
