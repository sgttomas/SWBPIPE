// Pure source/record arithmetic. No solver/model/runtime-memory experiment.
// Supply parsed H/observations/k6b/counts.jsonl as rows; return JSON-compatible data.
const derive = function(rows) {
  return rows.map(c=>({model:c.model,members:c.w1_members,free_dofs:c.w1_free_dofs,rows:c.w1_rows,profile_entries:c.w1_profile_entries,legacy_fixed_h:c.estimate_w1_fixed_bytes,legacy_emax_h:c.estimate_adm_bytes_w1a,phases:[ [256,48],[512,80],[1024,144] ].map(([p,w])=>{
    const f=c.w1_free_dofs,q=c.w1_rows,z=c.w1_profile_entries;
    const oldShift=3*q*(w+8)+2*z*w+f*(36+32+(w+8)+w);
    const newShift=3*q*w+2*z*w+f*(36+32+w)+3*f*w;
    return {precision:p,wide_bytes:w,old_shift_principal:oldShift,new_shift_principal:newShift,delta_shift_principal:newShift-oldShift,delta_vectors_replace_work:2*f*w,delta_option_principal:-8*(3*q+f),delta_report_options:-40*q,uc_c_at_bt_ct:4*f*w,uc_block_row_indices:4*f};
  })}));
};

