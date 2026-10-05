#[test]
fn rv60_repair_closed_keys_exact_entered_prefixes() {
 use super::retained_product::{AdapterWork,AdapterEvent as E,AdapterFault,CaptureError};
 let valid=serde_json::json!({"a":0,"b":0});
 let w=AdapterWork::default();w.rv60_closed_keys(&valid,&["a","b"]).unwrap();
 assert_eq!(w.counts.get(),[0,2,0,4,6,3,0,1,0,0]);
 for invalid in [serde_json::json!([]),serde_json::json!({"a":0})] {
  let w=AdapterWork::default();assert!(matches!(w.rv60_closed_keys(&invalid,&["a","b"]),Err(CaptureError::Association(_))));assert_eq!(w.counts.get(),[0,0,0,1,0,0,0,1,0,0]);
 }
 let w=AdapterWork::default();assert!(w.rv60_closed_keys(&serde_json::json!({"a":0,"c":0}),&["a","b"]).is_err());assert_eq!(w.counts.get(),[0,2,0,4,6,3,0,1,0,0]);
 for event in [E::ValidationEntry,E::KeyProbe,E::IdentityByteRead] {
  let w=AdapterWork::default();let mut counts=[0;10];counts[event as usize]=u64::MAX;w.counts.set(counts);
  assert!(matches!(w.rv60_closed_keys(&valid,&["a","b"]),Err(CaptureError::Accounting(AdapterFault::Overflow(e))) if e==event));
  assert_eq!(w.counts.get()[event as usize],u64::MAX);
  let expected=match event {E::ValidationEntry=>[0,0,0,u64::MAX,0,0,0,0,0,0],E::KeyProbe=>[0,1,0,1,0,u64::MAX,0,1,0,0],E::IdentityByteRead=>[0,1,0,2,u64::MAX,1,0,1,0,0],_=>unreachable!()};assert_eq!(w.counts.get(),expected);
 }
 println!("RV60_REPAIR_CLOSED_KEYS_PREFIXES true");
}
#[test]
fn rv60_repair_fixed_sign_exact_read_prefixes() {
 use super::retained_product::{AdapterWork,AdapterEvent as E,AdapterFault,CaptureError,rv60_mode_metadata};
 let (e,_)=observed(specimen(false));let row=&e.results[0];let case=&row.basis_ref.as_ref().unwrap().ref_id;
 let w=AdapterWork::default();rv60_mode_metadata(row,case,&w).unwrap();
 let bytes=2*(row.metadata.as_ref().unwrap().sign_convention.len()+row.id.len()) as u64;assert_eq!(w.counts.get(),[0,0,0,3,bytes,2,0,1,0,0]);
 let mut wrong=row.clone();wrong.metadata.as_mut().unwrap().sign_convention="wrong".into();
 let w=AdapterWork::default();assert_eq!(rv60_mode_metadata(&wrong,case,&w).unwrap_err().to_string(),"ordinary sparse mode sign");assert_eq!(w.counts.get(),[0,0,0,2,0,1,0,1,0,0]);
 let w=AdapterWork::default();let mut counts=[0;10];counts[E::IdentityByteRead as usize]=u64::MAX;w.counts.set(counts);
 assert!(matches!(rv60_mode_metadata(row,case,&w),Err(CaptureError::Accounting(AdapterFault::Overflow(E::IdentityByteRead)))));
 assert_eq!(w.counts.get(),[0,0,0,2,u64::MAX,1,0,1,0,0]);
 println!("RV60_REPAIR_SIGN_PREFIXES true");
}
