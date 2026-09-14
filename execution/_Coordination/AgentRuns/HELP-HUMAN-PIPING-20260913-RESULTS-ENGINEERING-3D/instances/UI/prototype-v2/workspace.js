/* Offline interaction design only. No Rust engine, hash proof or engineering validation. */
'use strict';
const $=id=>document.getElementById(id), esc=v=>String(v??'').replace(/[&<>"']/g,c=>({
  '&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'
}
[c])), clone=v=>JSON.parse(JSON.stringify(v));
const demoModel=()=>({
  nodes:[{
    id:'N1',p:[0,0,0]
  },{
    id:'N2',p:[2.4,0,0]
  },{
    id:'N3',p:[2.4,0,1.8]
  },{
    id:'N4',p:[4.8,0,1.8]
  },{
    id:'N5',p:[4.8,1.8,1.8]
  },{
    id:'N6',p:[6.2,1.8,1.8]
  },{
    id:'U1',p:[.4,3.1,0]
  },{
    id:'U2',p:[2.4,3.1,0]
  }],pipes:[{
    id:'P1',from:'N1',to:'N2',wall:6.02
  },{
    id:'P2',from:'N2',to:'N3',wall:6.02
  },{
    id:'P3',from:'N3',to:'N4',wall:6.02
  },{
    id:'P4',from:'N4',to:'N5',wall:6.02
  },{
    id:'P5',from:'N5',to:'N6',wall:6.02
  },{
    id:'PFREE',from:'U1',to:'U2',wall:6.02
  }],components:[],supports:[{
    id:'S1',node:'N1',family:'Anchor',dofs:['Ux','Uy','Uz','Rx','Ry','Rz'],provenance:'Invented demonstration restraint'
  }]
});
const S={
  model:demoModel(),rev:1,history:[],redo:[],selection:{
    type:'Node',ref:'N5'
  },group:'Build',tool:'route',row:1,phase:'ready',draft:null,frozen:null,lastRoute:null,tab:'review',engine:'ready',result:null,saved:null,resultSelection:0,camera:{
    yaw:-.65,pitch:.65,zoom:1
  },density:'comfortable',sequence:1,operationScenario:'ready',solveScenario:'current',message:'Design simulation · no Rust operation or hash proof',generation:0
};
const unitFactor={
  m:1,mm:.001,ft:.3048,in:.0254
};
function announce(t){
  S.message=t;
  $('live-message').textContent=t;
}
function selectEntity(type,ref){
  S.selection={
    type,ref
  };
  $('selection-label').textContent=`${type} ${ref}`;
  renderTree();
  renderCanvas();
  if(S.tool==='select')renderEditor();
}
function option(values,value){
  return values.map(v=>`<option value="${
  esc(v)
}" ${v===value?'selected':''}>${esc(v)}</option>`).join('');
}
function input(label,id,value='',type='text'){
  return `<label>${label}<input id="${
  id
}" value="${
  esc(value)
}" type="${
  type
}" ${type==='number'?'step="any"':''} data-testid="${
  id
}"></label>`;
}
function sel(label,id,values,value){
  return `<label>${label}<select id="${
  id
}" data-testid="${
  id
}">${option(values,value)}</select></label>`;
}
function newDraft(tool){
  const n=S.model.nodes.find(n=>n.id===S.selection.ref)||S.model.nodes.find(n=>n.id==='N5')||S.model.nodes[0];
  const p=S.model.pipes.find(p=>p.id===S.selection.ref)||S.model.pipes[4]||S.model.pipes[0];
  const next=S.sequence+6;
  switch(tool){
    case'route':return {
      from:n.id,endMode:'new',to:'N6',nodeId:`N${next}`,pipeId:`P${next}`,plane:'XZ',axis:'Free',unit:'m',x:'6.2',y:'1.8',z:'3',od:'114.3',wall:'6.02',sectionUnit:'mm',material:'M-DEMO',yx:'0',yy:'1',yz:'0',provenance:'Invented demo geometry; user may replace',placement:'Typed invented coordinates',continue:false
    };
    case'node':return {
      nodeId:`N${next}`,unit:'m',x:'6.2',y:'1.8',z:'3',plane:'XZ',axis:'Free',provenance:'Invented demo node geometry',placement:'Typed invented coordinates'
    };
    case'component':return{
      node:n.id,kind:S.row===14?'ExpansionJoint':'Valve',id:`C${S.sequence}`,primary:p.id,secondary:'',branch:'',mode:'symbol',length:'',unit:'mm',provenance:'Invented demo symbol; no catalog data'
    };
    case'support':return{
      node:n.id,id:`S${S.sequence+1}`,family:S.row===12?'SpringHanger':S.row===13?'Guide':'Anchor',dofs:[],stiffness:'',stiffnessUnit:'N/m',stiffnessDOF:'Ux',provenance:'',mode:'create',behavior:S.row===13?'Gap':'Linear',gap:'',friction:'',hangerType:'Variable',hotLoad:'',coldLoad:'',travel:''
    };
    case'property':return{
      pipe:p.id,field:'wall_thickness',value:String(p.wall),unit:'mm',provenance:''
    };
    case'transform':return{
      pipe:'P5',action:'copy',dx:'0',dy:'1',dz:'0',angle:'0',unit:'m',provenance:'Invented unreferenced-run transform'
    };
    case'split':return{
      pipe:'PFREE',fraction:'0.5',provenance:'Invented split coordinates'
    };
    default:return{
    };
  }
}
function activate(row,tool){
  S.row=row;
  const cap=CAPABILITIES.find(c=>c.row===row);
  S.tool=tool||cap?.prototypeSurface||'mapped-inspector';
  if(S.tool==='history'){
    S.tab='history';
    renderDock();
    return;
  }
  if(S.tool==='review'){
    S.tab='review';
    renderDock();
    return;
  }
  S.draft=newDraft(S.tool);
  S.frozen=null;
  S.lastRoute=null;
  S.phase='draft';
  S.engine='ready';
  S.generation++;
  $('tool-search').value='';
  S.group=cap?.group||'Build';
  renderAll();
  announce(`${cap?.label||'Node'} · explicit design draft; model unchanged`);
}
function renderPalette(){
  $('groups').innerHTML=['Build','Supports','Properties','Loads','Edit','Select and View','Review'].map(g=>`<button data-group="${
  g
}" class="${
  g===S.group?'active':''
}">${g}</button>`).join('');
  const q=$('tool-search').value.toLowerCase();
  const visible=CAPABILITIES.filter(c=>q?`${c.row} ${c.label} ${c.mapping} ${c.class}`.toLowerCase().includes(q):c.group===S.group);
  $('tools').innerHTML=(S.group==='Properties'&&!q?'<button data-property="true">Edit pipe properties</button>':'')+(S.group==='Build'&&!q?'<button data-node="true">＋ Node</button>':'')+visible.map(c=>`<button data-capability="${
  c.row
}" class="${
  c.class==='ROADMAP'?'roadmap':c.row===S.row?'active':''
}" title="${
  esc(c.mapping)
} · ${
  esc(c.sourceResidual)
}">${esc(c.label)}${c.class==='ROADMAP'?'<span class="tool-number">ROADMAP</span>':''}</button>`).join('');
}
function renderTree(){
  const q=$('model-search').value.toLowerCase();
  const buckets=[['Nodes','Node',S.model.nodes,'○'],['Pipe runs','Element',S.model.pipes,'╱'],['Components','Component',S.model.components,'◇'],['Supports','Support',S.model.supports,'⊥']];
  $('tree').innerHTML=buckets.map(([label,type,entities,icon])=>`<div class="tree-group">⌄ ${label} <span class="badge">${entities.length}</span></div>${entities.filter(e=>`${type} ${e.id}`.toLowerCase().includes(q)).map(e=>`<button class="tree-item ${
  S.selection.ref===e.id&&S.selection.type===type?'active':''
}" data-type="${
  type
}" data-ref="${
  e.id
}" data-testid="tree-${
  e.id
}"><span class="entity-icon" aria-hidden="true">${icon}</span>${e.id}<small>${type==='Node'?e.p.map(v=>v.toFixed(1)).join(', '):type==='Element'?`${e.from} → ${e.to}`:e.kind||e.family||''}</small></button>`).join('')}`).join('');
  $('revision').textContent=`mock:r${S.rev}`;
}
function renderEditor(){
  const d=S.draft||newDraft(S.tool);
  S.draft=d;
  const cap=CAPABILITIES.find(c=>c.row===S.row);
  $('task-title').textContent=S.tool==='property'?'Pipe properties':S.tool==='node'?'Create node':cap?.label||'Route pipe';
  $('inspector-title').textContent=({
    route:'Route / straight run',node:'Build / explicit node',component:'Component / incidence',support:'Support / boundary',property:'Properties / pipe',transform:'Edit / run transform',split:'Build / split',select:'Selection / identity',units:'View / readout units'
  })[S.tool]||'Command / mapped context';
  let h='';
  if(['route','node'].includes(S.tool)){
    h='<div class="section-caption">Explicit invented demo inputs</div>';
    if(S.tool==='route'){
      h+=sel('From · existing node','from',S.model.nodes.map(n=>n.id),d.from)+`<div class="field-row">${sel('Endpoint','endMode',['new','existing'],d.endMode)}${input('Pipe identity','pipeId',d.pipeId)}</div>`;
      if(d.endMode==='existing')h+=sel('To · existing node','to',S.model.nodes.map(n=>n.id),d.to);
    }
    if(S.tool==='node'||d.endMode==='new'){
      h+=`<div class="field-row">${input('New node identity','nodeId',d.nodeId)}${sel('Coordinate unit','unit',['m','mm','ft','in'],d.unit)}</div><div class="field-row">${sel('Pointer plane','plane',['XY','XZ','YZ'],d.plane)}${sel('Pointer constraint','axis',['Free',...d.plane.split('')],d.axis)}</div><div class="xyz">${input('X','x',d.x,'number')}${input('Y','y',d.y,'number')}${input('Z','z',d.z,'number')}</div><p id="placement-provenance">${esc(d.placement)}. Click a blank plane to capture; drag beyond 4 CSS px to orbit. Typed XYZ remains authored.</p>`;
    }
    if(S.tool==='route')h+=`<h3>Pipe section · entered values</h3><div class="field-row">${input('Outside diameter','od',d.od,'number')}${input('Wall thickness','wall',d.wall,'number')}</div><div class="field-row">${sel('Section unit','sectionUnit',['mm','m','ft','in'],d.sectionUnit)}${sel('Material record','material',['M-DEMO'],d.material)}</div><p>M-DEMO is an invented material reference; engineering values missing. No protected catalogue.</p><div class="section-caption">Explicit local Y reference</div><div class="xyz">${input('Yx','yx',d.yx,'number')}${input('Yy','yy',d.yy,'number')}${input('Yz','yz',d.yz,'number')}</div><label class="check"><input id="continue-check" type="checkbox" ${d.continue?'checked':''}> Enable continuation after matching apply</label>`;
    h+=input('Source / provenance','provenance',d.provenance);
  }
  else if(S.tool==='component'){
    h='<div class="section-caption">Symbol placement · no finite geometry</div>'+sel('At node','node',S.model.nodes.map(n=>n.id),d.node)+sel('Component kind','kind',['Bend','Tee','Reducer','Valve','Flange','ExpansionJoint'],d.kind)+input('Component identity','id',d.id)+sel('Primary / inlet pipe','primary',incident(d.node),d.primary)+sel('Secondary / outlet pipe','secondary',['',...incident(d.node)],d.secondary)+sel('Branch role (tee only)','branch',['',...incident(d.node)],d.branch)+sel('Insertion behavior','mode',['symbol','finite'],d.mode)+input('Finite face-to-face (missing)','length',d.length,'number')+sel('Entered length unit','unit',['mm','m','in','ft'],d.unit)+input('Source / provenance','provenance',d.provenance)+'<div class="warning">Finite insert-in-run is unavailable / backfill. Symbols do not infer connected-network geometry or connector properties.</div>';
  }
  else if(S.tool==='support'){
    h='<div class="section-caption">Explicit engineering inputs required</div>'+sel('At node','node',S.model.nodes.map(n=>n.id),d.node)+`<div class="field-row">${input('Support identity','id',d.id)}${sel('Action','mode',['create','update'],d.mode)}</div>`+sel('Named family','family',['Anchor','Guide','LineStop','VerticalSupport','SpringHanger'],d.family)+'<h3>Restrained DOFs · none inferred</h3><div class="xyz">'+['Ux','Uy','Uz','Rx','Ry','Rz'].map(v=>`<label class="check"><input type="checkbox" data-dof="${
  v
}" ${d.dofs.includes(v)?'checked':''}>${v}</label>`).join('')+'</div><div class="field-row">'+input('Optional stiffness','stiffness',d.stiffness,'number')+sel('Stiffness DOF','stiffnessDOF',['Ux','Uy','Uz','Rx','Ry','Rz'],d.stiffnessDOF)+'</div>'+sel('Stiffness unit','stiffnessUnit',['N/m','N*m/rad'],d.stiffnessUnit);
    if(S.row===12||d.family==='SpringHanger')h+=sel('Hanger type','hangerType',['Variable','ConstantEffort'],d.hangerType)+`<div class="field-row">${input('Hot load · N','hotLoad',d.hotLoad,'number')}${input('Cold load · N','coldLoad',d.coldLoad,'number')}</div>`+input('Travel · mm','travel',d.travel,'number')+'<p>No imported hanger catalogue is bundled. Limits and verified source remain required production inputs.</p>';
    if(S.row===13)h+=sel('Nonlinear behavior','behavior',['Gap','LiftOff','OneWay','Friction'],d.behavior)+`<div class="field-row">${input('Gap · mm','gap',d.gap,'number')}${input('Friction ratio','friction',d.friction,'number')}</div>`;
    h+=input('Source / provenance (required)','provenance',d.provenance)+'<p>Blank values stay missing. This prototype never supplies inferred engineering stiffness or restraints.</p>';
  }
  else if(S.tool==='property')h='<div class="section-caption">Pipe properties</div>'+sel('Pipe','pipe',S.model.pipes.map(p=>p.id),d.pipe)+'<label>Property<select id="field" data-testid="field"><option value="wall_thickness">Wall thickness</option></select></label>'+input('Wall thickness','value',d.value,'number')+sel('Entered unit','unit',['mm'],d.unit)+input('Source / provenance (required)','provenance',d.provenance)+'<p>Exact before/after is frozen on Add. M-DEMO has no accepted material values.</p>';
  else if(S.tool==='transform')h='<div class="section-caption">Explicit source set / no cascade</div>'+sel('Source pipe','pipe',S.model.pipes.map(p=>p.id),d.pipe)+sel('Transform','action',['copy','rotate','mirror'],d.action)+`<div class="xyz">${input('ΔX','dx',d.dx,'number')}${input('ΔY','dy',d.dy,'number')}${input('ΔZ','dz',d.dz,'number')}</div>`+input('Rotation · rad','angle',d.angle,'number')+sel('Translation unit','unit',['m','mm','ft','in'],d.unit)+input('Source / provenance','provenance',d.provenance)+'<div class="warning">P1–P5 belong to the attached model: transform blocked. PFREE is an invented unreferenced run. No connected-network transform semantics are implied.</div>';
  else if(S.tool==='split')h=sel('Explicit source pipe','pipe',S.model.pipes.map(p=>p.id),d.pipe)+input('Split fraction (0–1)','fraction',d.fraction,'number')+input('Source / provenance','provenance',d.provenance)+'<p>Maps to split_pipe_run with explicit new node, original pipe on first half and one new pipe identity. Only simple unreferenced demo run is simulated.</p>';
  else if(S.tool==='select')h='<div class="section-caption">View state / no operation</div>'+sel('Typed entity','selection-ref',[...S.model.nodes,...S.model.pipes,...S.model.supports,...S.model.components].map(e=>e.id),S.selection.ref)+`<pre>${esc(JSON.stringify(S.selection,null,2))}</pre><p>Tree, canvas and typed selector share identity. Selection never creates a model checkpoint.</p>`;
  else if(S.tool==='units')h='<div class="section-caption">Display preference / no mutation</div>'+sel('Readout system','readout',['Entered','SI','US'],$('display-units').value)+'<p>Prototype length readouts only: invented m ↔ ft and mm ↔ in. Other unavailable conversions retain entered value with explanation. Draft and stored model inputs never change.</p>';
  else h=`<div class="section-caption">${esc(cap?.class||'DESIGN SIMULATION')}</div><h3>${esc(cap?.label||'Command')}</h3><p>Current typed context: <strong>${esc(S.selection.type)} ${esc(S.selection.ref)}</strong></p><div class="context-note">${esc(cap?.mapping||'No executable semantics')}</div><p>${esc(cap?.sourceResidual||'')}</p><div class="warning">${cap?.class==='ROADMAP'?'ROADMAP · future owner promotion required. No executable action.':'Mapped contextual inspector only. Detailed authoring is outside these three prototype journeys; no simulated apply is offered.'}</div>`;
  $('editor').innerHTML=h;
  const active=['route','node','component','support','property','transform','split'].includes(S.tool);
  $('add-draft').disabled=!active||S.phase==='busy';
  $('add-draft').textContent='Add → Review';
  $('editor-message').textContent=S.phase==='draft'?'Enter explicit values, then freeze this draft.':S.phase==='cancelled'?'Draft withdrawn. Model and history unchanged.':S.phase==='applied'?'One simulated checkpoint applied. Continue or start another draft.':'Review is frozen. Changes require adding a new draft.';
  updateStateChrome();
}
function incident(node){
  return S.model.pipes.filter(p=>p.from===node||p.to===node).map(p=>p.id);
}
function readForm(){
  if(!S.draft)return;
  for(const el of $('editor').querySelectorAll('input[id],select[id]')){
    if(el.id==='continue-check')S.draft.continue=el.checked;
    else if(el.id in S.draft)S.draft[el.id]=el.value;
  }
  if(S.tool==='support')S.draft.dofs=[...$('editor').querySelectorAll('[data-dof]:checked')].map(e=>e.dataset.dof);
}
function baseIntent(kind,type,ref,path,after,unit='none',dimension='dimensionless',before='not_present'){
  return {
    operation_id:`op:prototype-${S.sequence}-${kind}`,operation_kind:kind==='connect_pipe_run'?'connect':kind.startsWith('create_')?'create':kind==='insert_component_symbol'?'insert':'modify',operation_status:'proposed',author_type:'user',target:{
      object_type:type,ref
    },change:{
      change_id:`change:prototype-${S.sequence}-${kind}`,change_kind:kind,field_label:'Explicit prototype authoring',field_path:path,before,after:typeof after==='string'?after:JSON.stringify(after),unit,dimension,source_note:S.draft.provenance||'Missing source; prototype blocks preparation'
    },validation:{
      schema_validation:'NOT_RUN_PROTOTYPE_ONLY',constraint_validation:'NOT_RUN_PROTOTYPE_ONLY',unit_validation:'NOT_RUN_PROTOTYPE_ONLY',diff_preview_status:'SIMULATED_ONLY',application_status:'not_applied'
    },audit_boundary:{
      mutation_route:'structured_operations_only',direct_model_mutation_allowed:false,requires_user_acceptance:true,mutates_accepted_model_state:false
    },professional_boundary:{
      human_review_required:true,software_makes_compliance_claim:false,software_makes_certification_claim:false,software_makes_sealing_claim:false,software_makes_approval_claim:false,software_makes_authentication_claim:false
    },rationale:'Interaction design simulation; source operation mapping only, no schema/Rust validation or model-hash proof'
  };
}
function freezeDraft(){
  readForm();
  const d=clone(S.draft),operations=[],errors=[];
  const need=(condition,msg)=>{
    if(!condition)errors.push(msg);
  };
  const finite=v=>v!==''&&Number.isFinite(Number(v));
  const allIds=[...S.model.nodes,...S.model.pipes,...S.model.supports,...S.model.components].map(e=>e.id);
  if('provenance'in d)need(d.provenance.trim(),'Enter explicit source / provenance.');
  if(['route','node'].includes(S.tool)){
    if(S.tool==='node'||d.endMode==='new'){
      need([d.x,d.y,d.z].every(finite),'Enter finite X, Y and Z values.');
      need(d.nodeId&&!allIds.includes(d.nodeId),'New node identity must be unique.');
      operations.push(baseIntent('create_node','Node',d.nodeId,'nodes',{
        id:d.nodeId,label:d.nodeId,position:{
          x:+d.x,y:+d.y,z:+d.z
        },provenance:d.provenance
      },d.unit,'length'));
    }
    if(S.tool==='route'){
      need(S.model.nodes.some(n=>n.id===d.from),'Choose resolved From node.');
      need(d.pipeId&&!allIds.includes(d.pipeId)&&d.pipeId!==d.nodeId,'New pipe identity must be unique.');
      need(d.endMode==='new'||(d.to!==d.from&&S.model.nodes.some(n=>n.id===d.to)),'Choose a different existing To node.');
      need(finite(d.od)&&finite(d.wall)&&+d.od>2*+d.wall&&+d.wall>0,'Enter positive valid section dimensions.');
      need([d.yx,d.yy,d.yz].every(finite)&&[d.yx,d.yy,d.yz].some(v=>+v!==0),'Explicit local Y reference must be finite and nonzero.');
      operations.push(baseIntent('connect_pipe_run','Element',d.pipeId,'pipe_segments',{
        id:d.pipeId,label:d.pipeId,from:d.from,to:d.endMode==='new'?d.nodeId:d.to,section:{
          outside_diameter:{
            value:+d.od,unit:d.sectionUnit
          },wall_thickness:{
            value:+d.wall,unit:d.sectionUnit
          }
        },material:d.material,y_reference:{
          x:+d.yx,y:+d.yy,z:+d.yz
        },provenance:d.provenance
      },d.sectionUnit,'length'));
    }
  }
  else if(S.tool==='component'){
    need(d.mode!=='finite','FINITE-INSERT-UNAVAILABLE: finite insert-in-run requires production backfill.');
    need(d.id&&!allIds.includes(d.id),'Component identity must be unique.');
    need(incident(d.node).includes(d.primary),'Choose an explicit incident primary pipe.');
    need(!d.secondary||incident(d.node).includes(d.secondary),'Secondary pipe must be incident to selected node.');
    need(d.kind!=='Tee'||(d.secondary&&d.branch&&new Set([d.primary,d.secondary,d.branch]).size===3&&incident(d.node).includes(d.branch)),'A tee needs three distinct explicit incident roles; demo node has no three-pipe tee.');
    operations.push(baseIntent('insert_component_symbol','Component',d.id,'components',{
      id:d.id,kind:d.kind,node:d.node,incident_roles:{
        primary:d.primary,secondary:d.secondary||null,branch:d.branch||null
      },insertion:'symbol_only',provenance:d.provenance
    }));
  }
  else if(S.tool==='support'){
    need(d.dofs.length||finite(d.stiffness),'Choose restrained DOFs or enter explicit stiffness.');
    if(d.stiffness){
      need(finite(d.stiffness)&&+d.stiffness>0,'Stiffness must be positive.');
      need(d.stiffnessUnit===(d.stiffnessDOF[0]==='R'?'N*m/rad':'N/m'),'Stiffness DOF and dimension must agree.');
    }
    if(d.family==='SpringHanger'){
      need([d.hotLoad,d.coldLoad,d.travel].every(finite),'Enter explicit cold/hot load and travel; no catalogue defaults.');
    }
    if(S.row===13){
      need(finite(d.gap),'Enter explicit gap.');
      if(d.behavior==='Friction')need(finite(d.friction),'Enter explicit friction ratio.');
    }
    need(d.mode==='update'?S.model.supports.some(s=>s.id===d.id):d.id&&!allIds.includes(d.id),d.mode==='update'?'Select an existing support identity for update.':'Support identity must be unique.');
    operations.push(baseIntent(d.mode==='update'?'update_support':'create_support','Support',d.id,'supports',{
      id:d.id,node:d.node,family:d.family,restrained_dofs:d.dofs,stiffness:d.stiffness?{
        dof:d.stiffnessDOF,value:+d.stiffness,unit:d.stiffnessUnit
      }
      :null,hanger:d.family==='SpringHanger'?{
        type:d.hangerType,hot_load:{
          value:+d.hotLoad,unit:'N'
        },cold_load:{
          value:+d.coldLoad,unit:'N'
        },travel:{
          value:+d.travel,unit:'mm'
        }
      }
      :null,nonlinear:S.row===13?{
        behavior:d.behavior,gap:{
          value:+d.gap,unit:'mm'
        },friction_ratio:d.friction?+d.friction:null
      }
      :null,provenance:d.provenance
    }));
  }
  else if(S.tool==='property'){
    const p=S.model.pipes.find(p=>p.id===d.pipe);
    need(p&&finite(d.value)&&+d.value>0&&+d.value<57.15,'Enter positive wall thickness below half invented OD.');
    operations.push(baseIntent('set_field','Element',d.pipe,'section.wall_thickness',String(d.value),d.unit,'length',String(p?.wall)));
  }
  else if(S.tool==='transform'){
    need(d.pipe==='PFREE','ATTACHED-TRANSFORM-BLOCKED: attached nodes/supports/components are outside transform scope. No cascade.');
    need([d.dx,d.dy,d.dz,d.angle].every(finite),'Enter finite transform values.');
    operations.push(baseIntent('transform_pipe_run','Element',d.pipe,'geometry_transform',{
      source_pipe_refs:[d.pipe],action:d.action,translation:{
        x:+d.dx,y:+d.dy,z:+d.dz,unit:d.unit
      },angle:{
        value:+d.angle,unit:'rad'
      },new_identities:{
        from:`U${S.sequence}a`,to:`U${S.sequence}b`,pipe:`PCOPY${S.sequence}`
      },local_y_reference:{
        x:0,y:1,z:0
      },provenance:d.provenance
    },d.unit,'length'));
  }
  else if(S.tool==='split'){
    need(d.pipe==='PFREE','SPLIT-CONTEXT-BLOCKED: prototype only simulates PFREE.');
    need(finite(d.fraction)&&+d.fraction>0&&+d.fraction<1,'Split fraction must be inside (0,1).');
    operations.push(baseIntent('split_pipe_run','Element',d.pipe,'pipe_segments',{
      source_pipe_ref:d.pipe,fraction:+d.fraction,new_node_id:`SPLITN${S.sequence}`,new_pipe_ids:[d.pipe,`SPLITB${S.sequence}`],provenance:d.provenance
    }));
  }
  S.frozen={
    mockBasis:`mock:r${S.rev}`,basis:S.rev,generation:S.generation,tool:S.tool,draft:d,submission:operations.length>1?{
      batch_id:`batch:prototype-${S.sequence}`,operations
    }
    :{
      intent:operations[0]
    },errors,memberList:operations.map(o=>o.operation_id)
  };
  S.phase=errors.length?'blocked':'review';
  S.tab='review';
  announce(errors.length?errors[0]:'Frozen draft prepared. Inspect and validate simulation before Apply.');
  renderAll();
}
function validateSimulation(){
  if(!S.frozen)return;
  if(S.frozen.errors.length){
    S.phase='blocked';
  }
  else if(S.operationScenario==='stale'||S.frozen.basis!==S.rev){
    S.phase='stale';
    announce('STALE-REVIEW: mock basis changed. Add a fresh draft.');
  }
  else if(S.operationScenario==='blocked'){
    S.phase='blocked';
    announce('MOCK-VALIDATION-BLOCKED: injected diagnostic; no publication.');
  }
  else if(S.operationScenario==='cancelled'){
    cancelDraft();
    return;
  }
  else if(S.operationScenario==='busy'){
    S.phase='busy';
    S.engine='busy';
    S.heldGeneration=S.generation;
    announce('Mock operation held busy. Cancel withdraws this generation.');
  }
  else{
    S.phase='ready';
    announce('Simulation review ready. No Rust/schema validation performed.');
  }
  renderAll();
}
function invalidateResults(){
  if(S.result?.status==='Current')S.result={
    ...S.result,status:'Unavailable',overlay:false,readiness:false,diagnostic:'Model changed; current result invalidated. Saved example may be reopened as Historical.'
  };
}
function checkpoint(model,label){
  S.history.push({
    model:clone(S.model),label,sequence:S.sequence,fromRev:S.rev
  });
  if(S.history.length>25)S.history.shift();
  S.redo=[];
  S.model=model;
  S.rev++;
  S.sequence++;
  invalidateResults();
}
function applyFrozen(){
  const f=S.frozen;
  if(!f||S.phase!=='ready')return;
  if(f.basis!==S.rev||S.operationScenario==='stale'){
    S.phase='stale';
    announce('STALE-REVIEW: exact frozen mock basis no longer current.');
    renderAll();
    return;
  }
  if(S.operationScenario!=='ready'){
    validateSimulation();
    return;
  }
  S.phase='busy';
  S.engine='busy';
  const d=f.draft,m=clone(S.model),factor=unitFactor[d.unit]||1;
  const ops=f.submission.operations||[f.submission.intent];
  for(const o of ops){
    const k=o.change.change_kind,a=JSON.parse(k==='set_field'?JSON.stringify(o.change.after):o.change.after);
    if(k==='create_node')m.nodes.push({
      id:a.id,p:[a.position.x,a.position.y,a.position.z].map(v=>v*factor)
    });
    if(k==='connect_pipe_run')m.pipes.push({
      id:a.id,from:a.from,to:a.to,wall:a.section.wall_thickness.value*unitFactor[a.section.wall_thickness.unit]*1000
    });
    if(k==='insert_component_symbol')m.components.push({
      id:a.id,node:a.node,kind:a.kind,roles:a.incident_roles
    });
    if(k==='create_support')m.supports.push({
      id:a.id,node:a.node,family:a.family,dofs:a.restrained_dofs,provenance:a.provenance
    });
    if(k==='update_support')Object.assign(m.supports.find(s=>s.id===a.id),{
      node:a.node,family:a.family,dofs:a.restrained_dofs,provenance:a.provenance
    });
    if(k==='set_field')m.pipes.find(p=>p.id===d.pipe).wall=+o.change.after;
    if(k==='transform_pipe_run'){
      const p=m.pipes.find(p=>p.id===d.pipe),from=m.nodes.find(n=>n.id===p.from),to=m.nodes.find(n=>n.id===p.to),ids=a.new_identities;
      const transform=n=>{
        const q=n.p.map((v,i)=>v+[+d.dx,+d.dy,+d.dz][i]*factor);
        if(d.action==='rotate'){
          const x=q[0],y=q[1],c=Math.cos(+d.angle),s=Math.sin(+d.angle);
          q[0]=x*c-y*s;
          q[1]=x*s+y*c;
        }
        if(d.action==='mirror')q[1]=-q[1];
        return q;
      };
      m.nodes.push({
        id:ids.from,p:transform(from)
      },{
        id:ids.to,p:transform(to)
      });
      m.pipes.push({
        ...p,id:ids.pipe,from:ids.from,to:ids.to
      });
    }
    if(k==='split_pipe_run'){
      const p=m.pipes.find(p=>p.id===a.source_pipe_ref),n1=m.nodes.find(n=>n.id===p.from),n2=m.nodes.find(n=>n.id===p.to);
      m.nodes.push({
        id:a.new_node_id,p:n1.p.map((v,i)=>v+(n2.p[i]-v)*a.fraction)
      });
      m.pipes=m.pipes.filter(e=>e.id!==p.id);
      m.pipes.push({
        ...p,id:a.new_pipe_ids[0],to:a.new_node_id
      },{
        ...p,id:a.new_pipe_ids[1],from:a.new_node_id
      });
    }
  }
  checkpoint(m,`${f.tool}: ${ops.map(o=>o.change.change_kind).join(' + ')}`);
  S.phase='applied';
  S.engine='ready';
  if(f.tool==='route')S.lastRoute={
    end:d.endMode==='new'?d.nodeId:d.to,pipe:d.pipeId,matchingRevision:S.rev,enabled:d.continue,draft:d,memberList:f.memberList
  };
  announce(`Applied one simulated atomic checkpoint · mock:r${S.rev}. No accepted engineering mutation.`);
  renderAll();
}
function cancelDraft(){
  S.generation++;
  S.frozen=null;
  S.phase='cancelled';
  S.engine='ready';
  S.lastRoute=null;
  announce('Cancelled / withdrawn generation. No model publication; late mock completion cannot resurrect it.');
  renderAll();
}
function continueRoute(){
  const r=S.lastRoute;
  if(!r||!r.enabled||r.matchingRevision!==S.rev||S.phase!=='applied')return;
  S.tool='route';
  S.row=1;
  S.draft={
    ...clone(r.draft),from:r.end,nodeId:`N${S.sequence+6}`,pipeId:`P${S.sequence+6}`,x:'',y:'',z:'',placement:'Continuation after own matching apply; enter next explicit XYZ'
  };
  S.phase='draft';
  S.frozen=null;
  S.lastRoute=null;
  S.generation++;
  renderAll();
  announce(`Continue From ${S.draft.from} · matching applied route only.`);
}
function historyMove(direction){
  if(S.phase==='busy'||S.engine==='busy')return;
  const source=direction==='undo'?S.history:S.redo,target=direction==='undo'?S.redo:S.history;
  if(!source.length)return;
  const item=source.pop();
  target.push({
    model:clone(S.model),label:item.label,sequence:S.sequence,fromRev:S.rev
  });
  S.model=clone(item.model);
  S.rev++;
  S.frozen=null;
  S.lastRoute=null;
  S.phase='ready';
  S.generation++;
  invalidateResults();
  S.draft=newDraft(S.tool);
  renderAll();
  announce(`${direction==='undo'?'Undo':'Redo'} one simulated session checkpoint · mock:r${S.rev}; Current results invalidated.`);
}
function updateStateChrome(){
  const b=$('draft-badge');
  b.textContent=S.phase;
  b.className='badge '+S.phase;
  $('engine-status').textContent=S.engine;
  $('pending-count').textContent=S.frozen&&S.phase!=='applied'?String(S.frozen.memberList.length):'0';
  $('apply').disabled=S.phase!=='ready'||!S.frozen||S.engine==='busy';
  $('continue').disabled=!S.lastRoute?.enabled||S.phase!=='applied'||S.lastRoute.matchingRevision!==S.rev;
  $('undo').disabled=!S.history.length||S.phase==='busy'||S.engine==='busy';
  $('redo').disabled=!S.redo.length||S.phase==='busy'||S.engine==='busy';
  const rs=S.result?.status||'No result';
  $('result-status').textContent=rs;
  $('result-status').className='badge '+rs.toLowerCase();
  for(const tab of ['review','results','history','state'])$('tab-'+tab).setAttribute('aria-selected',String(S.tab===tab));
}
function renderDock(){
  updateStateChrome();
  if(S.tab==='review'){
    const f=S.frozen;
    if(!f){
      $('dock-body').innerHTML='<div class="empty"><span class="empty-icon" aria-hidden="true">▤</span><div><strong>Prepare a change, inspect it, then apply</strong><p>The canvas and form share one draft. Add freezes entered values and a mock revision.<br>Apply publishes one simulated session checkpoint; engineering acceptance remains outside this prototype.</p><div class="steps"><span>1 · Enter</span><span>2 · Review</span><span>3 · Simulate validation</span><span>4 · Apply</span></div></div></div>';
      return;
    }
    const ops=f.submission.operations||[f.submission.intent];
    $('dock-body').innerHTML=`<div class="review-layout"><div><div class="review-head"><h3>${S.phase==='applied'?'Applied simulated checkpoint':'Frozen draft · '+S.phase}</h3><span>Basis ${f.mockBasis} · ${f.memberList.length} member(s)</span></div>${f.errors.length?`<div class="warning">${f.errors.map(esc).join('<br>')}</div>`:''}<table><thead><tr><th>Typed target</th><th>Operation reference</th><th>Before → proposed</th><th>Unit</th></tr></thead><tbody>${ops.filter(Boolean).map(o=>`<tr><td>${esc(o.target.object_type)} ${esc(o.target.ref)}</td><td>${esc(o.change.change_kind)}</td><td>${o.change.change_kind==='set_field'?esc(o.change.before+' → '+o.change.after):'not present → explicit authored record'}</td><td>${esc(o.change.unit)}</td></tr>`).join('')}</tbody></table><details><summary>Exact source-shaped mock payload · NOT schema/Rust validated</summary><pre id="intent-json">${esc(JSON.stringify(f.submission,null,2))}</pre></details></div><div class="review-summary"><strong>One atomic simulated change</strong><p>Frozen members, entered values and ${f.mockBasis}.</p><p>State: <b>${esc(S.phase)}</b> · acceptance: no engineering claim.</p><button id="validate-simulation" data-testid="validate-simulation" ${['busy','applied','blocked','stale','cancelled'].includes(S.phase)?'disabled':''}>Validate simulation</button><p>No model hash, Rust proof, numerical check or professional approval.</p>${S.phase==='stale'?'<div class="warning">STALE-REVIEW · Add again from current basis.</div>':S.phase==='busy'?'<div class="warning">BUSY · Cancel withdraws generation.</div>':''}</div></div>`;
  }
  else if(S.tab==='history')$('dock-body').innerHTML=`<div class="review-head"><h3>Session checkpoints · ${S.history.length} earlier / ${S.redo.length} later</h3><span>Camera, selection, density and units never enter history</span></div><table><thead><tr><th>Step</th><th>Simulated change</th><th>Original mock basis</th></tr></thead><tbody>${S.history.map((h,i)=>`<tr><td>${i+1}</td><td>${esc(h.label)}</td><td>mock:r${h.fromRev}</td></tr>`).join('')}</tbody></table>${S.history.length?'':'<p class="context-note">No earlier session checkpoint. Drafts and cancelled requests do not mutate model.</p>'}`;
  else if(S.tab==='state')$('dock-body').innerHTML=`<div class="state-layout"><pre id="state-json">${esc(JSON.stringify(inspect(),null,2))}</pre><pre id="full-intent-json">${esc(JSON.stringify(S.frozen?.submission||{note:'No frozen draft'},null,2))}</pre></div>`;
  else renderResults();
}
const resultRows=()=>[{
  quantity:'Displacement',kind:'translation',component:'Uz',value:2.31,unit:'mm',dimension:'length',frame:'global',location:'Node N5',basis:`mock:r${S.rev}`,ref:'N5',type:'Node'
},{
  quantity:'Rotation',kind:'rotation',component:'Ry',value:.00124,unit:'rad',dimension:'angle',frame:'global',location:'Node N5',basis:`mock:r${S.rev}`,ref:'N5',type:'Node'
},{
  quantity:'Reaction',kind:'force',component:'Fz',value:1240,unit:'N',dimension:'force',frame:'global',location:'Support S1 / N1',basis:`mock:r${S.rev}`,ref:'S1',type:'Support'
},{
  quantity:'Moment',kind:'moment',component:'My',value:84.2,unit:'N*m',dimension:'moment',frame:'local pipe',location:'Pipe P3 / end',basis:`mock:r${S.rev}`,ref:'P3',type:'Element'
},{
  quantity:'Stress',kind:'stress',component:'bending',value:12.8,unit:'MPa',dimension:'stress',frame:'local pipe',location:'Pipe P3 / section',basis:`mock:r${S.rev}`,ref:'P3',type:'Element'
},{
  quantity:'Illustrative ratio',kind:'ratio',component:'demand / entered limit',value:.42,unit:'1',dimension:'dimensionless',frame:'not applicable',location:'Pipe P3 / mock check',basis:`mock:r${S.rev}`,ref:'P3',type:'Element'
},{
  quantity:'Work residual',kind:'diagnostic',component:'iteration residual',value:.018,unit:'N*m',dimension:'not classified by prototype',frame:'solver diagnostic',location:'Mock solve / iteration 4',basis:`mock:r${S.rev}`,ref:null,type:null
}];
function solve(){
  if(S.engine==='busy'){
    announce('Mock engine is busy; cancel held scenario first.');
    return;
  }
  const outcome=S.solveScenario;
  if(outcome==='current'){
    S.result={
      status:'Current',basis:`mock:r${S.rev}`,rows:resultRows(),overlay:true,readiness:false,acceptance:'UNKNOWN',diagnostic:'Invented example only · no engineering rule acceptance',invented:true
    };
    S.saved=clone(S.result);
    try{
      localStorage.setItem('ops-design-saved-example',JSON.stringify(S.saved));
    }
    catch{
    }
    announce('Simulated Current example generated and saved as a prototype record. Values are invented.');
  }
  else{
    S.result={
      status:outcome==='blocked'?'Blocked':outcome==='nonconverged'?'Nonconverged':outcome==='cancelled'?'Cancelled':'Busy',basis:`mock:r${S.rev}`,rows:[],overlay:false,readiness:false,acceptance:'UNKNOWN',diagnostic:outcome==='blocked'?'MOCK-MISSING-INPUT: accepted material values remain missing; no solved export.':outcome==='nonconverged'?'MOCK-NONCONVERGED: iteration limit reached; no Current result or solved export.':outcome==='cancelled'?'MOCK-CANCELLED: solve withdrawn; no Current result.':'MOCK-BUSY: solve held for cancel demonstration.',invented:true
    };
    if(outcome==='busy')S.engine='busy';
    announce(S.result.diagnostic);
  }
  S.tab='results';
  renderAll();
}
function reopen(){
  if(!S.saved){
    try{
      S.saved=JSON.parse(localStorage.getItem('ops-design-saved-example'));
    }
    catch{
    }
  }
  if(!S.saved){
    announce('No saved prototype example. Run simulated Current first.');
    return;
  }
  S.result={
    ...clone(S.saved),status:'Historical',overlay:false,readiness:false,acceptance:'UNKNOWN',diagnostic:'Reopened invented prototype record. Historical excludes Current overlay, rule checks and readiness; no production checksum interpretation.'
  };
  S.tab='results';
  announce('Historical prototype record · acceptance UNKNOWN · excluded from Current overlays and rule checks.');
  renderAll();
}
function readout(row){
  const preference=$('display-units').value;
  if(preference==='US'&&row.dimension==='length')return {
    value:(row.value/25.4).toFixed(4),unit:'in',note:'Prototype arithmetic length readout only; entered mm preserved.'
  };
  if(preference==='SI'&&row.dimension==='length')return{
    value:(row.value/1000).toFixed(5),unit:'m',note:'Prototype arithmetic length readout only; entered mm preserved.'
  };
  return{
    value:row.value,unit:row.unit,note:preference==='Entered'?'Entered units shown.':'Conversion unavailable in prototype for this quantity; entered value retained.'
  };
}
function renderResults(){
  const r=S.result;
  const controls=`<div class="result-controls"><span>${r?`<strong>${esc(r.status)}</strong> · ${esc(r.basis)} · invented example`:'No simulated result yet'}</span><div><button id="solve" class="primary" data-testid="solve" ${S.engine==='busy'?'disabled':''}>Run simulated solve</button> <button id="reopen" data-testid="reopen">Reopen saved prototype</button> <button id="cancel-solve" ${S.engine==='busy'?'':'disabled'}>Cancel solve</button></div></div>`;
  if(!r||!r.rows.length){
    $('dock-body').innerHTML=controls+`<div class="empty"><span class="empty-icon">≋</span><div><strong>${esc(r?.status||'Inspect explicit engineering quantities')}</strong><p>${esc(r?.diagnostic||'Generate invented rows to examine quantity, kind, unit, dimension, frame, location and basis.')}<br>No successful solved export is available in this prototype.</p></div></div>`;
    return;
  }
  const row=r.rows[Math.min(S.resultSelection,r.rows.length-1)],read=readout(row);
  $('dock-body').innerHTML=controls+`<div class="result-layout"><div>${r.status==='Historical'||r.status==='Unavailable'?`<div class="warning">${esc(r.diagnostic)} Acceptance ${esc(r.acceptance)}. Current overlay: OFF · rule readiness: OFF.</div>`:''}<table data-testid="results-table"><thead><tr><th>Quantity / kind</th><th>Location</th><th>Value</th><th>Unit</th><th>Inspect</th></tr></thead><tbody>${r.rows.map((q,i)=>{const v=readout(q);return`<tr><td>${esc(q.quantity)} <small>(${q.kind})</small></td><td>${esc(q.location)}</td><td>${esc(v.value)}</td><td>${esc(v.unit)}</td><td><button data-result="${
  i
}" ${i===S.resultSelection?'class="active"':''}>Details${q.ref?' ↗':''}</button></td></tr>`;}).join('')}</tbody></table></div><div class="quantity-detail"><strong>${esc(row.quantity)} · source fields</strong><dl>${['kind','component','unit','dimension','frame','location','basis'].map(k=>`<dt>${k}</dt><dd>${esc(row[k])}</dd>`).join('')}</dl><p>${esc(read.note)}</p><p>Classification comes from explicit fields, never identifier. Work residual is diagnostic N*m; no new energy dimension policy.</p><p>Acceptance ${esc(r.acceptance)} · all numerical examples invented.</p></div></div>`;
}
function project(p){
  const [x,y,z]=p.map((v,i)=>v-[3.1,1.3,.9][i]);
  const {
    yaw,pitch,zoom
  }
  =S.camera;
  const a=x*Math.cos(yaw)-y*Math.sin(yaw),b=x*Math.sin(yaw)+y*Math.cos(yaw);
  return[450+a*76*zoom,295+(b*Math.sin(pitch)-z*Math.cos(pitch))*76*zoom];
}
function svgLine(a,b,attrs=''){
  const p=project(a),q=project(b);
  return `<line x1="${
  p[0]
}" y1="${
  p[1]
}" x2="${
  q[0]
}" y2="${
  q[1]
}" ${attrs}/>`;
}
function renderCanvas(){
  let h='<g stroke="#d9e1dc" stroke-width=".8">';
  for(let i=-2;i<=9;i++)h+=svgLine([i,-2,0],[i,6,0])+svgLine([-2,i,0],[9,i,0]);
  h+='</g>';
  const nodes=new Map(S.model.nodes.map(n=>[n.id,n]));
  const current=S.result?.status==='Current'&&S.result.overlay;
  if(current){
    h+='<g stroke="#7aaa88" stroke-width="2" opacity=".65" stroke-dasharray="4 4">';
    for(const p of S.model.pipes){
      const a=nodes.get(p.from),b=nodes.get(p.to);
      if(a&&b)h+=svgLine(a.p.map((v,i)=>v+(i===2?.12:0)),b.p.map((v,i)=>v+(i===2?.12:0)));
    }
    h+='</g>';
  }
  for(const p of S.model.pipes){
    const a=nodes.get(p.from),b=nodes.get(p.to);
    if(!a||!b)continue;
    const selected=S.selection.type==='Element'&&S.selection.ref===p.id;
    h+=svgLine(a.p,b.p,`stroke="${
  selected?'#d79c3b':'#637f85'
}" stroke-width="${
  selected?13:10
}" stroke-linecap="round" data-entity="${
  p.id
}" data-type="Element"`)+svgLine(a.p,b.p,`stroke="${
  selected?'#e9ba6c':'#91a7aa'
}" stroke-width="3" stroke-linecap="round" pointer-events="none"`);
    const mid=project(a.p.map((v,i)=>(v+b.p[i])/2));
    h+=`<text x="${
  mid[0]+10
}" y="${
  mid[1]-12
}">${p.id}</text>`;
  }
  for(const n of S.model.nodes){
    const p=project(n.p),selected=S.selection.type==='Node'&&S.selection.ref===n.id;
    h+=`<circle cx="${
  p[0]
}" cy="${
  p[1]
}" r="${
  selected?8:4
}" fill="${
  selected?'#e2ac4f':'#eaf0ed'
}" stroke="${
  selected?'#b67c28':'#728d94'
}" stroke-width="2" data-entity="${
  n.id
}" data-type="Node"/><text class="node-label ${
  selected?'selected-text':''
}" x="${
  p[0]-12
}" y="${
  p[1]+24
}">${n.id}</text>`;
  }
  for(const s of S.model.supports){
    const n=nodes.get(s.node);
    if(!n)continue;
    const p=project(n.p);
    h+=`<g data-entity="${
  s.id
}" data-type="Support" stroke="#6f8584" stroke-width="2" fill="#dce6df"><path d="M${
  p[0]
} ${
  p[1]+8
}l-12 20h24z"/><path d="M${
  p[0]-18
} ${
  p[1]+31
}h36"/><text x="${
  p[0]+20
}" y="${
  p[1]+30
}">${s.id} · ${esc(s.family)}</text></g>`;
  }
  for(const c of S.model.components){
    const n=nodes.get(c.node);
    if(!n)continue;
    const p=project(n.p);
    h+=`<g data-entity="${
  c.id
}" data-type="Component" fill="#e3ecef" stroke="#527a8a" stroke-width="2"><path d="M${
  p[0]-10
} ${
  p[1]-9
}l20 18v-18l-20 18z"/><text x="${
  p[0]+15
}" y="${
  p[1]-15
}">${c.id} · ${esc(c.kind)}</text></g>`;
  }
  const d=S.draft;
  if(d&&['route','node'].includes(S.tool)&&S.phase!=='cancelled'&&S.phase!=='applied'){
    const start=S.tool==='route'?nodes.get(d.from):null;
    const point=S.tool==='route'&&d.endMode==='existing'?nodes.get(d.to)?.p:[+d.x,+d.y,+d.z].map(v=>v*(unitFactor[d.unit]||1));
    if(point&&point.every(Number.isFinite)){
      if(start)h+=svgLine(start.p,point,'stroke="#2d84c4" stroke-width="4" stroke-dasharray="9 6"');
      const p=project(point);
      h+=`<circle cx="${
  p[0]
}" cy="${
  p[1]
}" r="8" fill="#d5e9f7" stroke="#2d84c4" stroke-width="2"/><path d="M${
  p[0]-15
} ${
  p[1]
}h30 M${
  p[0]
} ${
  p[1]-15
}v30" stroke="#2d84c4" stroke-width="1"/><text class="selected-text" x="${
  p[0]+14
}" y="${
  p[1]-16
}">Draft ${esc(d.nodeId||d.to)}</text>`;
    }
  }
  h+=`<text class="dimension-label" x="${
  project([1.2,0,0])[0]
}" y="${
  project([1.2,0,0])[1]+48
}">2.400 m · entered demo</text>`;
  $('canvas').innerHTML=h;
  $('canvas-hint').textContent=current?'Current invented overlay · scale ×50 illustrative':d&&['route','node'].includes(S.tool)?`Draft plane ${d.plane} · ${d.axis} · ${d.placement}`:'Drag to orbit · canvas and tree share typed selection';
  $('view-state').textContent=`${S.result?.status==='Historical'?'Historical overlay excluded · ':''}Camera is view state · invented geometry`;
}
function renderAll(){
  renderPalette();
  renderTree();
  renderEditor();
  renderCanvas();
  renderDock();
  $('selection-label').textContent=`${S.selection.type} ${S.selection.ref}`;
}
function inspect(){
  return{
    claim:'SIMULATED DESIGN PROTOTYPE',revision:`mock:r${S.rev}`,selection:clone(S.selection),phase:S.phase,engine:S.engine,tool:S.tool,generation:S.generation,draft:clone(S.draft),frozen:S.frozen?{
      basis:S.frozen.mockBasis,members:clone(S.frozen.memberList),errors:clone(S.frozen.errors),submission:clone(S.frozen.submission)
    }
    :null,lastRoute:clone(S.lastRoute),historyCount:S.history.length,redoCount:S.redo.length,nodeCount:S.model.nodes.length,pipeCount:S.model.pipes.length,componentCount:S.model.components.length,supportCount:S.model.supports.length,result:S.result?{
      status:S.result.status,basis:S.result.basis,overlay:S.result.overlay,readiness:S.result.readiness,acceptance:S.result.acceptance,rows:clone(S.result.rows)
    }
    :null,camera:clone(S.camera),density:S.density,operationScenario:S.operationScenario,solveScenario:S.solveScenario,message:S.message
  };
}
window.prototypeInspect=inspect;
// Read-only witness hook; no state setter.
$('groups').addEventListener('click',e=>{
  const b=e.target.closest('[data-group]');if(b){
    S.group=b.dataset.group;$('tool-search').value='';renderPalette();
  }
});
$('tools').addEventListener('click',e=>{
  const b=e.target.closest('[data-capability],[data-node],[data-property]');if(!b)return;if(b.dataset.property){activate(17,'property');return;}if(b.dataset.node){
    activate(1,'node');return;
  }
  const key=b.dataset.capability;activate(/^\d+$/.test(key)?+key:key);
});
$('tool-search').addEventListener('input',renderPalette);
$('model-search').addEventListener('input',renderTree);
$('tree').addEventListener('click',e=>{
  const b=e.target.closest('[data-ref]');if(b)selectEntity(b.dataset.type,b.dataset.ref);
});
$('editor').addEventListener('input',e=>{
  readForm();if(S.phase!=='draft'){
    S.frozen=null;S.phase='draft';S.generation++;
  }
  if(e.target.id==='selection-ref'){
    const ref=e.target.value;selectEntity(S.model.nodes.some(n=>n.id===ref)?'Node':S.model.pipes.some(p=>p.id===ref)?'Element':S.model.supports.some(s=>s.id===ref)?'Support':'Component',ref);
  }
  if(e.target.id==='readout'){
    $('display-units').value=e.target.value;renderDock();
  }
  if(['x','y','z'].includes(e.target.id)){
    S.draft.placement='Typed explicit XYZ';$('placement-provenance')&&($('placement-provenance').textContent='Typed explicit XYZ · pointer plane/constraint affects only later pointer capture.');
  }
  updateStateChrome();renderCanvas();
});
$('editor').addEventListener('change',e=>{
  readForm();if(e.target.id==='plane'&&!['Free',...S.draft.plane.split('')].includes(S.draft.axis))S.draft.axis='Free';if(e.target.id==='node'&&S.tool==='component'){
    S.draft.primary=incident(S.draft.node)[0]||'';S.draft.secondary='';S.draft.branch='';
  }
  if(e.target.id==='pipe'&&S.tool==='property')S.draft.value=String(S.model.pipes.find(p=>p.id===S.draft.pipe)?.wall||'');if(['plane','endMode','node','family','pipe','mode'].includes(e.target.id))renderEditor();renderCanvas();
});
$('add-draft').onclick=freezeDraft;
$('cancel-draft').onclick=cancelDraft;
$('apply').onclick=applyFrozen;
$('continue').onclick=continueRoute;
$('undo').onclick=()=>historyMove('undo');
$('redo').onclick=()=>historyMove('redo');
$('dock-body').addEventListener('click',e=>{
  if(e.target.id==='validate-simulation')validateSimulation();if(e.target.id==='solve')solve();if(e.target.id==='reopen')reopen();if(e.target.id==='cancel-solve'){
    S.engine='ready';S.result={
      status:'Cancelled',rows:[],overlay:false,readiness:false,acceptance:'UNKNOWN',diagnostic:'Mock solve cancelled; no publication.'
    };renderAll();announce('Cancelled held mock solve; no Current result.');
  }
  const b=e.target.closest('[data-result]');if(b){
    S.resultSelection=+b.dataset.result;const q=S.result.rows[S.resultSelection];if(q.ref)selectEntity(q.type,q.ref);renderDock();
  }
});
for(const t of ['review','results','history','state'])$('tab-'+t).onclick=()=>{
  S.tab=t;
  renderDock();
};
$('history-open').onclick=()=>{
  S.tab='history';
  renderDock();
};
$('density').onchange=()=>{
  S.density=$('density').value;
  document.body.dataset.density=S.density;
  announce(`${S.density} density · view state only; same draft/history.`);
};
$('display-units').onchange=()=>{
  renderDock();
  announce(`Readouts ${$('display-units').value}. Entered draft/model units preserved; unavailable conversions explained.`);
};
for(const b of document.querySelectorAll('[data-view]'))b.onclick=()=>{
  S.camera={
    yaw:b.dataset.view==='front'?0:-.65,pitch:b.dataset.view==='top'?Math.PI/2:b.dataset.view==='front'?0:.65,zoom:1
  };
  document.querySelectorAll('[data-view]').forEach(x=>x.classList.toggle('active',x===b));
  renderCanvas();
};
$('fit').onclick=()=>{
  S.camera.zoom=1;
  renderCanvas();
  announce('Fit model · no history change.');
};
const toggleTree=()=>document.body.classList.toggle('model-hidden');
$('tree-toggle').onclick=toggleTree;
$('model-show').onclick=toggleTree;
$('scenarios-toggle').onclick=()=>{
  $('scenarios').showModal();
};
$('scenarios-close').onclick=()=>{
  $('scenarios').close();
  $('scenarios-toggle').focus();
};
$('scenarios').addEventListener('close',()=>$('scenarios-toggle').focus());
$('operation-scenario').onchange=()=>{
  S.operationScenario=$('operation-scenario').value;
  if(S.phase==='busy'&&S.operationScenario==='ready'){
    S.phase=S.frozen?'review':'draft';
    S.engine='ready';
  }
  if(S.operationScenario==='stale'&&S.frozen&&S.phase!=='applied')S.phase='stale';
  if(S.operationScenario==='busy'&&S.frozen&&S.phase!=='applied'){
    S.phase='busy';
    S.engine='busy';
    S.heldGeneration=S.generation;
  }
  if(S.operationScenario==='cancelled')cancelDraft();
  renderAll();
};
$('complete-held').onclick=()=>{
  if(S.heldGeneration!==S.generation||!S.frozen){
    announce('LATE-MOCK-COMPLETION-IGNORED: withdrawn generation cannot restore a review.');
    return;
  }
  S.engine='ready';
  S.phase='review';
  S.operationScenario='ready';
  $('operation-scenario').value='ready';
  renderAll();
  announce('Held mock completion matched current generation; returned to Review, no model apply.');
};
$('solve-scenario').onchange=()=>S.solveScenario=$('solve-scenario').value;
$('reset').onclick=()=>{
  S.model=demoModel();
  S.rev=1;
  S.history=[];
  S.redo=[];
  S.result=null;
  S.saved=null;
  S.sequence=1;
  S.operationScenario='ready';
  S.solveScenario='current';
  S.engine='ready';
  S.selection={
    type:'Node',ref:'N5'
  };
  $('operation-scenario').value='ready';
  $('solve-scenario').value='current';
  try{
    localStorage.removeItem('ops-design-saved-example');
  }
  catch{
  }
  $('scenarios').close();
  activate(1);
};
let gesture=null;
$('canvas').addEventListener('pointerdown',e=>{
  gesture={
    x:e.clientX,y:e.clientY,lastX:e.clientX,lastY:e.clientY,drag:false,id:e.pointerId,target:e.target.closest('[data-entity]')
  };$('canvas').setPointerCapture(e.pointerId);
});
$('canvas').addEventListener('pointermove',e=>{
  if(!gesture)return;const g=gesture;if(Math.hypot(e.clientX-g.x,e.clientY-g.y)>4)g.drag=true;if(g.drag){
    S.camera.yaw+=(e.clientX-g.lastX)*.008;S.camera.pitch=Math.max(-1.45,Math.min(1.45,S.camera.pitch+(e.clientY-g.lastY)*.006));renderCanvas();
  }
  g.lastX=e.clientX;g.lastY=e.clientY;
});
$('canvas').addEventListener('pointerup',e=>{
  const g=gesture;gesture=null;if(!g)return;if(g.drag||Math.hypot(e.clientX-g.x,e.clientY-g.y)>4){
    announce('Orbit drag exceeded 4 CSS px. No point or model change.');return;
  }
  if(g.target){
    selectEntity(g.target.dataset.type,g.target.dataset.entity);announce(`Selected typed ${g.target.dataset.type} ${g.target.dataset.entity}. Plane placement is distinct.`);return;
  }
  if(!['route','node'].includes(S.tool)||S.phase==='busy'||S.phase==='applied'||S.phase==='cancelled'||S.tool==='route'&&S.draft.endMode!=='new')return;const d=S.draft,start=S.model.nodes.find(n=>n.id===d.from)?.p||[0,0,0],screenPoint=new DOMPoint(e.clientX,e.clientY).matrixTransform($('canvas').getScreenCTM().inverse()),sx=screenPoint.x,sy=screenPoint.y,axes=d.plane.split('').map(a=>({
    X:0,Y:1,Z:2
  })[a]),base=start.slice();const o=project(base),u=base.slice(),v=base.slice();u[axes[0]]++;v[axes[1]]++;const pu=project(u),pv=project(v),a=pu[0]-o[0],b=pv[0]-o[0],c=pu[1]-o[1],f=pv[1]-o[1],det=a*f-b*c;if(Math.abs(det)<.01){
    announce('POINTER-PLANE-EDGE-ON: select Iso or use typed XYZ.');return;
  }
  let t=((sx-o[0])*f-(sy-o[1])*b)/det,w=((sy-o[1])*a-(sx-o[0])*c)/det;if(d.axis!== 'Free'){
    if(axes[0]!==({
      X:0,Y:1,Z:2
    })[d.axis])t=0;if(axes[1]!==({
      X:0,Y:1,Z:2
    })[d.axis])w=0;
  }
  base[axes[0]]+=t;base[axes[1]]+=w;for(let i=0;i<3;i++)d[['x','y','z'][i]]=String(Number((base[i]/unitFactor[d.unit]).toPrecision(12)));d.placement=`Pointer ${d.plane} / ${d.axis}; 12 significant digits in ${d.unit}`;S.phase='draft';S.frozen=null;S.generation++;renderAll();announce('Pointer captured into the same authoritative XYZ draft; no model change.');
});
$('canvas').addEventListener('pointercancel',()=>{
  gesture=null;
});
$('canvas').addEventListener('wheel',e=>{
  e.preventDefault();S.camera.zoom=Math.max(.5,Math.min(1.7,S.camera.zoom-e.deltaY*.0008));renderCanvas();
},{
  passive:false
});
const resize=$('dock-body').parentElement.querySelector('.dock-resize');
let dockGesture=null;
resize.onpointerdown=e=>{
  dockGesture={
    y:e.clientY,h:document.querySelector('.dock').getBoundingClientRect().height
  };
  resize.setPointerCapture(e.pointerId);
};
resize.onpointermove=e=>{
  if(dockGesture)document.documentElement.style.setProperty('--dock',Math.max(150,Math.min(window.innerHeight*.45,dockGesture.h+dockGesture.y-e.clientY))+'px');
};
resize.onpointerup=()=>dockGesture=null;
resize.onkeydown=e=>{
  if(['ArrowUp','ArrowDown'].includes(e.key)){
    e.preventDefault();
    const h=document.querySelector('.dock').getBoundingClientRect().height;
    document.documentElement.style.setProperty('--dock',Math.max(150,Math.min(window.innerHeight*.45,h+(e.key==='ArrowUp'?20:-20)))+'px');
  }
};
document.addEventListener('keydown',e=>{
  if(e.key==='Escape'&&!$('scenarios').open){
    cancelDraft();$('add-draft').focus();
  }
  if(['INPUT','SELECT','TEXTAREA'].includes(e.target.tagName))return;if(e.key==='/'){
    e.preventDefault();$('tool-search').focus();
  }
  if(e.key.toLowerCase()==='f')$('fit').click();if((e.metaKey||e.ctrlKey)&&e.key.toLowerCase()==='z'){
    e.preventDefault();historyMove(e.shiftKey?'redo':'undo');
  }
  if(e.key.toLowerCase()==='r'&&!e.metaKey&&!e.ctrlKey)activate(1);if(e.key==='Enter'&&e.target===$('canvas')){
    $('editor').querySelector('input,select')?.focus();
  }
});
S.draft=newDraft('route');
renderAll();
