# core/product_physics/src/lib.rs

Absolute source origin: /Users/ryan/.codex/worktrees/8728/chirality-engineering-contracts-20260913/projects/chirality-piping/core/product_physics/src/lib.rs
SHA256: 399096ec0f8033ae9bd9dcfd933091b0e0e482eb1f0107caf8af1098879e123b


1920:                 metadata: None,
1921:             });
1922:         }
1923:         component_stress_modifier_count += append_component_stress_multiplier_results(
1924:             &mut results,
1925:             diagnostics,
1926:             model,
1927:             &pipe.element_id,
1928:             &end_i_stress,
1929:             &end_j_stress,
1930:             include_pressure_longitudinal,
1931:         );
1932:     }
1933: 
1934:     require_finite_mechanics(results.iter().map(|row| row.value))?;
1935:     Ok(LoadCaseSolve {
1936:         load_case_id: load_case.id.clone(),
1937:         results,
1938:         max_displacement,
1939:         max_stress,
1940:         component_stress_modifier_count,
1941:         component_pressure_thrust_load_count,
1942:         support_force_vectors,

7630:     stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
7631:     include_pressure_longitudinal: bool,
7632: ) -> Option<f64> {
7633:     if !stress.findings.is_empty() {
7634:         return None;
7635:     }
7636:     let components = &stress.components;
7637:     let axial = components.axial_normal.unwrap_or(0.0);
7638:     let pressure_longitudinal = if include_pressure_longitudinal {
7639:         components.pressure_longitudinal.unwrap_or(0.0)
7640:     } else {
7641:         0.0
7642:     };
7643:     let bending_y = components.bending_normal_y.unwrap_or(0.0).abs();
7644:     let bending_z = components.bending_normal_z.unwrap_or(0.0).abs();
7645:     let base_normal = axial + pressure_longitudinal;
7646:     let bending_total = bending_y + bending_z;
7647:     Some(
7648:         (base_normal + bending_total)
7649:             .abs()
7650:             .max((base_normal - bending_total).abs())
7651:             / 1_000_000.0,
7652:     )
7653: }
7654: 
7655: fn append_component_stress_multiplier_results(
7656:     results: &mut Vec<ResultItem>,
7657:     diagnostics: &mut Vec<Diagnostic>,
7658:     model: &PreviewModel,
7659:     pipe_id: &str,
7660:     end_i_stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
7661:     end_j_stress: &open_pipe_stress_stress_recovery::StressRecoveryResult,
7662:     include_pressure_longitudinal: bool,
7663: ) -> usize {
7664:     let Some(pipe) = model
7665:         .pipe_segments
7666:         .iter()
7667:         .find(|candidate| candidate.id == pipe_id)
7668:     else {
7669:         return 0;
7670:     };
7671:     let endpoint_stresses = [
7672:         ("end_i", pipe.from.as_str(), end_i_stress),
7673:         ("end_j", pipe.to.as_str(), end_j_stress),
7674:     ];
7675:     let mut appended = 0;
7676:     for (location, node_id, stress) in endpoint_stresses {
7677:         let Some(base_value_mpa) = open_formula_summary_mpa(stress, include_pressure_longitudinal)
7678:         else {
7679:             continue;
7680:         };
7681:         for component in model
7682:             .components
7683:             .iter()
7684:             .filter(|component| component.node == node_id)
7685:         {
7686:             let Some(modifier) = component_stress_modifier_for_pipe(component, pipe_id) else {
7687:                 continue;
7688:             };
7689:             append_component_stress_multiplier_result(
7690:                 results,
7691:                 diagnostics,
7692:                 component,
7693:                 pipe_id,
7694:                 location,
7695:                 base_value_mpa,
7696:                 modifier,
7697:             );
7698:             appended += 1;
7699:         }
7700:     }
7701:     appended
7702: }
7703: 

7728:     None
7729: }
7730: 
7731: fn bend_stress_modifier(component: &PreviewComponent) -> Option<ComponentStressModifier<'_>> {
7732:     let solver_consumption = component
7733:         .mechanics_interface
7734:         .as_ref()
7735:         .and_then(|interface| interface.solver_consumption.as_deref())
7736:         .unwrap_or("mechanics_geometry_only");
7737:     let flexibility_in_assembled_stiffness =
7738:         solver_consumption == DEC_070_CURVED_BEND_SOLVER_CONSUMPTION;
7739:     if solver_consumption != "mechanics_geometry_only" && !flexibility_in_assembled_stiffness {
7740:         return None;
7741:     }
7742:     let modifiers = component.modifiers.as_ref()?;
7743:     let sif = modifiers.sif_user_value.as_ref()?.value;
7744:     let flexibility = modifiers.flexibility_factor_user_value.as_ref()?.value;
7745:     if !positive_finite(sif) || !positive_finite(flexibility) {
7746:         return None;
7747:     }
7748:     let source_reference = modifiers
7749:         .source_reference
7750:         .as_deref()
7751:         .filter(|value| !value.trim().is_empty())
7752:         .unwrap_or("source_reference_missing");
7753:     Some(ComponentStressModifier {
7754:         family: "bend",
7755:         side: "through",
7756:         sif,
7757:         flexibility,
7758:         flexibility_in_assembled_stiffness,
7759:         source_reference,
7760:         solver_consumption,
7761:     })
7762: }
7763: 
7764: fn branch_stress_modifier_for_pipe<'a>(
7765:     component: &'a PreviewComponent,
7766:     pipe_id: &str,
7767: ) -> Option<ComponentStressModifier<'a>> {
7768:     let solver_consumption = component
7769:         .mechanics_interface
7770:         .as_ref()
7771:         .and_then(|interface| interface.solver_consumption.as_deref())
7772:         .unwrap_or("mechanics_geometry_only");
7773:     if solver_consumption != "mechanics_geometry_only" {
7774:         return None;
7775:     }
7776:     let geometry = component.geometry.as_ref()?;
7777:     let modifiers = component.modifiers.as_ref()?;
7778:     let (side, sif) = if geometry
7779:         .branch_header_pipe_ref
7780:         .as_deref()
7781:         .filter(|value| *value == pipe_id)
7782:         .is_some()
7783:     {
7784:         (
7785:             "header",
7786:             modifiers.branch_header_sif_user_value.as_ref()?.value,
7787:         )
7788:     } else if geometry
7789:         .branch_branch_pipe_ref
7790:         .as_deref()
7791:         .filter(|value| *value == pipe_id)
7792:         .is_some()
7793:     {
7794:         (
7795:             "branch",
7796:             modifiers.branch_branch_sif_user_value.as_ref()?.value,
7797:         )
7798:     } else {
7799:         return None;
7800:     };
7801:     let flexibility = modifiers.flexibility_factor_user_value.as_ref()?.value;
7802:     if !positive_finite(sif) || !positive_finite(flexibility) {
7803:         return None;
7804:     }
7805:     let source_reference = modifiers
7806:         .source_reference
7807:         .as_deref()
7808:         .filter(|value| !value.trim().is_empty())
7809:         .unwrap_or("source_reference_missing");
7810:     Some(ComponentStressModifier {
7811:         family: "branch",
7812:         side,
7813:         sif,
7814:         flexibility,
7815:         flexibility_in_assembled_stiffness: false,
7816:         source_reference,
7817:         solver_consumption,
7818:     })

7821: fn append_component_stress_multiplier_result(
7822:     results: &mut Vec<ResultItem>,
7823:     diagnostics: &mut Vec<Diagnostic>,
7824:     component: &PreviewComponent,
7825:     pipe_id: &str,
7826:     location: &str,
7827:     base_value_mpa: f64,
7828:     modifier: ComponentStressModifier<'_>,
7829: ) {
7830:     let component_suffix = stable_suffix(&component.id);
7831:     let pipe_suffix = stable_suffix(pipe_id);
7832:     let endpoint = endpoint_id_location(location);
7833:     let result_id =
7834:         format!("result:stress:{component_suffix}:{pipe_suffix}:{endpoint}:user-multiplier");
7835:     // DEC-070 no-double-counting rule: when the flexibility factor is realized
7836:     // in the assembled curved-bend macro-element stiffness, the review
7837:     // multiplier applies the user-entered SIF only; the legacy
7838:     // mechanics_geometry_only mode keeps sif * flexibility byte-identically.
7839:     let multiplier = if modifier.flexibility_in_assembled_stiffness {
7840:         modifier.sif
7841:     } else {
7842:         modifier.sif * modifier.flexibility
7843:     };
7844:     let value = base_value_mpa * multiplier;
7845:     let mut basis = format!(
7846:         "component_family={};component_side={};user_entered_sif={};user_entered_flexibility={};source={};solver_consumption={}",
7847:         modifier.family,
7848:         modifier.side,
7849:         rounded_scalar(modifier.sif),
7850:         rounded_scalar(modifier.flexibility),
7851:         modifier.source_reference,
7852:         modifier.solver_consumption
7853:     );
7854:     if modifier.flexibility_in_assembled_stiffness {
7855:         basis.push_str(";flexibility_realization=assembled_curved_bend_macro_element_stiffness");
7856:     }
7857:     let sign_convention = if modifier.flexibility_in_assembled_stiffness {
7858:         "positive value is base open-mechanics stress summary multiplied by the user-entered SIF only; the user-entered flexibility factor enters the assembled curved-bend macro-element stiffness"
7859:     } else {
7860:         "positive value is base open-mechanics stress summary multiplied by user-entered component modifiers; base frame stiffness unchanged"
7861:     };
7862:     results.push(ResultItem {
7863:         id: result_id.clone(),
7864:         kind: "component_user_stress_multiplier_review".to_string(),
7865:         value,
7866:         unit: "MPa".to_string(),
7867:         entity_ref: component.id.clone(),
7868:         basis_ref: None,
7869:         source_result_refs: endpoint_stress_source_refs(pipe_id, location),
7870:         metadata: Some(ResultMetadata {
7871:             component: "user_entered_component_stress_multiplier".to_string(),
7872:             coordinate_system: "component_review".to_string(),
7873:             location: format!("{pipe_id}:{location}"),
7874:             basis,
7875:             sign_convention: sign_convention.to_string(),
7876:         }),
7877:     });
7878:     let message = if modifier.flexibility_in_assembled_stiffness {
7879:         format!(
7880:             "{} component {} applies user-entered {} SIF {} to {} {location} stress-recovery review; user-entered flexibility factor {} enters the assembled curved-bend macro-element stiffness; solver_consumption is {}; no protected or default component factor is supplied",
7881:             modifier.family,
7882:             component.id,
7883:             modifier.side,
7884:             rounded_scalar(modifier.sif),
7885:             pipe_id,
7886:             rounded_scalar(modifier.flexibility),
7887:             modifier.solver_consumption
7888:         )
7889:     } else {
7890:         format!(
7891:             "{} component {} applies user-entered {} SIF {} and flexibility factor {} to {} {location} stress-recovery review; solver_consumption remains {}; no protected or default component factor is supplied",
7892:             modifier.family,
7893:             component.id,
7894:             modifier.side,
7895:             rounded_scalar(modifier.sif),
7896:             rounded_scalar(modifier.flexibility),
7897:             pipe_id,
7898:             modifier.solver_consumption
7899:         )
7900:     };
7901:     diagnostics.push(diag(
7902:         &format!(
7903:             "diagnostic:component-stress-multiplier:{}:{}:{}",
7904:             component_suffix, pipe_suffix, endpoint
7905:         ),
7906:         "COMPONENT_STRESS_MULTIPLIER_APPLIED",
7907:         "info",
7908:         message,
7909:         vec![
7910:             component.id.clone(),
7911:             pipe_id.to_string(),
7912:             result_id,
7913:             modifier.source_reference.to_string(),
7914:         ],
7915:     ));
7916: }
7917: 
7918: fn append_expansion_joint_user_stiffness_results(

8233: fn endpoint_stress_source_refs(pipe_id: &str, location: &str) -> Vec<String> {
8234:     let suffix = stable_suffix(pipe_id);
8235:     let endpoint = endpoint_id_location(location);
8236:     [
8237:         format!("result:stress:{suffix}:{endpoint}:axial-normal"),
8238:         format!("result:stress:{suffix}:{endpoint}:bending-normal-y"),
8239:         format!("result:stress:{suffix}:{endpoint}:bending-normal-z"),
8240:         format!("result:stress:{suffix}:{endpoint}:torsional-shear"),
8241:         format!("result:stress:{suffix}"),
8242:     ]
8243:     .into_iter()
8244:     .collect()
8245: }
8246: 
8247: fn is_bend_component(component: &PreviewComponent) -> bool {
8248:     matches!(component.kind.as_str(), "bend" | "elbow")
8249: }
8250: 
8251: fn is_branch_component(component: &PreviewComponent) -> bool {
8252:     matches!(
8253:         component.kind.as_str(),
8254:         "branch" | "tee" | "branch_connection"
8255:     )
8256: }
8257: 
8258: fn is_expansion_joint_component(component: &PreviewComponent) -> bool {

12370:     #[test]
12371:     fn bend_component_user_multipliers_emit_stress_review_rows() {
12372:         let result = run_linear_static_preview(request());
12373:         let default_row_id = "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier";
12374:         let combination_row_id =
12375:             "result:combination:combination-C-OPER-ALT:stress:component-C-110:pipe-P-100:end-j:user-multiplier";
12376:         let default_row = result
12377:             .results
12378:             .iter()
12379:             .find(|item| item.id == default_row_id)
12380:             .expect("bend user multiplier row should be emitted for adjacent pipe endpoint");
12381:         let combination_row = result
12382:             .results
12383:             .iter()
12384:             .find(|item| item.id == combination_row_id)
12385:             .expect("bend user multiplier row should participate in explicit combinations");
12386: 
12387:         assert_eq!(result.summary.component_stress_modifier_count, 8);
12388:         assert_eq!(default_row.kind, "component_user_stress_multiplier_review");
12389:         assert_eq!(default_row.entity_ref, "component:C-110");
12390:         assert!(default_row.value > 0.0);
12391:         assert!(default_row
12392:             .source_result_refs
12393:             .contains(&"result:stress:pipe-P-100:end-j:axial-normal".to_string()));
12394:         assert!(default_row
12395:             .source_result_refs
12396:             .contains(&"result:stress:pipe-P-100:end-j:bending-normal-y".to_string()));
12397:         assert!(default_row
12398:             .source_result_refs
12399:             .contains(&"result:stress:pipe-P-100".to_string()));
12400:         let metadata = default_row
12401:             .metadata
12402:             .as_ref()
12403:             .expect("component multiplier row carries recovery metadata");
12404:         assert_eq!(
12405:             metadata.component,
12406:             "user_entered_component_stress_multiplier"
12407:         );
12408:         assert_eq!(metadata.coordinate_system, "component_review");
12409:         assert_eq!(metadata.location, "pipe:P-100:end_j");
12410:         assert!(metadata.basis.contains("user_entered_sif=1.15"));
12411:         assert!(metadata.basis.contains("user_entered_flexibility=1.08"));
12412:         assert!(metadata
12413:             .basis
12414:             .contains("source=invented_user_entered_preview_no_code_table"));
12415:         assert!(metadata
12416:             .basis
12417:             .contains("solver_consumption=mechanics_geometry_only"));
12418:         assert!(metadata
12419:             .sign_convention
12420:             .contains("base frame stiffness unchanged"));
12421: 
12422:         assert_eq!(

# apps/desktop/src/features/report/ReportPanel.tsx

Absolute source origin: /Users/ryan/.codex/worktrees/8728/chirality-engineering-contracts-20260913/projects/chirality-piping/apps/desktop/src/features/report/ReportPanel.tsx
SHA256: b3d3b01ed1b1475be912e7412d73619965f53c5ebdf1ae9315ffe890836a2fab


291: 
292: function selectedResultRefs(result: MechanicsResult): string[] {
293:   return [
294:     result.summary.max_displacement?.result_ref,
295:     result.summary.max_open_formula_stress?.result_ref,
296:     result.results.find((item) => item.kind === "component_user_stress_multiplier_review")?.id,
297:     result.results.find((item) => item.id === "result:force:pipe-P-120:axial")?.id,
298:     result.results.find((item) => item.id === "result:force:pipe-P-120:axial:end-j")?.id,
299:     result.results.find((item) => item.id === "result:force:pipe-P-120:midspan:axial")?.id,
300:     result.results.find((item) => item.id === "result:force:pipe-P-120:quarter-1:shear-y")?.id,

325: }
326: 
327: function reportComponentProvenance(model: PreviewModel, result: MechanicsResult) {
328:   return model.components.map((component) => {
329:     const modifierRows = result.results.filter(
330:       (item) => item.kind === "component_user_stress_multiplier_review" && item.entity_ref === component.id
331:     );
332:     const stiffnessRows = result.results.filter(
333:       (item) => item.kind === "component_user_stiffness_macro_element_review" && item.entity_ref === component.id
334:     );
335:     const pressureThrustRows = result.results.filter(
336:       (item) => item.kind === "expansion_joint_pressure_thrust_load_review" && item.entity_ref === component.id
337:     );

379: function reportComponentStressModifierEvidence(model: PreviewModel, result: MechanicsResult) {
380:   return result.results
381:     .filter((item) => item.kind === "component_user_stress_multiplier_review")
382:     .map((item) => {
383:       const component = model.components.find((candidate) => candidate.id === item.entity_ref);
384:       return {
385:         result_ref: item.id,
386:         component_ref: item.entity_ref,
387:         component_kind: component?.kind ?? "component",
388:         value: item.value,
389:         unit: item.unit,
390:         source_result_refs: item.source_result_refs ?? [],
391:         recovery_basis: item.metadata?.basis ?? "not provided",
392:         sign_convention: item.metadata?.sign_convention ?? "not provided",
393:         modifier_source_ref: component?.modifiers?.source_reference ?? "not provided",
394:         solver_consumption: component?.mechanics_interface?.solver_consumption ?? "not provided",
395:         private_payload_included: false,
396:         protected_content_included: false,
397:         release_or_professional_claim: false
398:       };
399:     });
400: }
401: 
